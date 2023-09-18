use futures::Future;
use interop_android::{error::{GlobalResult, GlobalError}, ContextedGlobal, env::AndroidEnv};
use jni::{JNIEnv, objects::JThrowable};

use log::debug;
use tesseract::Error as TError;
use jni::errors::Error as JError;

use crate::context::TesseractContext;

pub fn tesseractify_jni_error(error: JError) -> TError {
    TError::new(tesseract::error::ErrorKind::Weird, "JNI Error", error)
}

pub fn tesseractify_no_exception<T, I, F>(fun: F) -> tesseract::Result<T>
where
    I: Into<T>,
    F: FnOnce() -> jni::errors::Result<I>,
{
    match fun() {
        Err(err) => {
            Err(tesseractify_jni_error(err))
        }
        Ok(value) => Ok(value.into())
    }
}

pub fn tesseractify_exception<'a: 'b, 'b>(env: &'b JNIEnv<'a>, exception: JThrowable<'a>) -> jni::errors::Result<tesseract::Error> {
    let user_cancelled_clazz = env.find_class_android("one/tesseract/UserCancelledException")?;
    let this_clazz = env.get_object_class(exception)?;
    let is_cancelled = env.is_assignable_from(user_cancelled_clazz, this_clazz)?;
    //is_instance_of(exception, clazz)?;
    let message = env.call_method(exception, "getMessage", "()Ljava/lang/String;", &[])?.l()?;
    let message: String = env.get_string(message.into())?.into();

    debug!("PRINTME {}", &message);

    let kind = if is_cancelled {
        debug!("ITISCANCELLED");
        tesseract::ErrorKind::Cancelled
    } else {
        debug!("ITISWEIRD");
        tesseract::ErrorKind::Weird
    };

    Ok(TError::described(kind, &message))
}

pub fn tesseractify<T, I, F>(env: &JNIEnv, fun: F) -> tesseract::Result<T>
where
    I: Into<T>,
    F: FnOnce() -> jni::errors::Result<I>,
{
    match fun() {
        Err(jni::errors::Error::JavaException) => {
            let tesseract_error = tesseractify_no_exception( || {
                let exception = env.exception_occurred()?;
                env.exception_clear()?;
                
                tesseractify_exception(env, exception)
            })?;

            Err(tesseract_error)
        }
        other => {
            tesseractify_no_exception(|| {other})
        }
    }
}

pub fn tesseractify_global_result<T>(result: GlobalResult<T>) -> tesseract::Result<T> {
    match result {
        Ok(ok) => Ok(ok),
        Err(error) => {
            Err(match error {
                GlobalError::JniError(error) => tesseractify_jni_error(error),
                GlobalError::Exception(exception) => {
                    exception.do_in_tesseract_context(10, |env, exception| {
                        let throwable = JThrowable::from(exception);
                        tesseractify_exception(&env, throwable)
                    })?
                }
            })
        }
    }
}

pub async fn tesseractify_async<T, F>(fun: impl FnOnce() -> F) -> tesseract::Result<T>
    where F: Future<Output = GlobalResult<T>>, {
    match fun().await {
        Ok(ok) => Ok(ok),
        Err(error) => {
            Err(match error {
                GlobalError::JniError(error) => tesseractify_jni_error(error),
                GlobalError::Exception(exception) => {
                    exception.do_in_tesseract_context(10, |env, exception| {
                        let throwable = JThrowable::from(exception);
                        tesseractify_exception(&env, throwable)
                    })?
                }
            })
        }
    }
}
