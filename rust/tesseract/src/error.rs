use interop_android::error::{GlobalResult, GlobalError};
use jni::{JNIEnv, objects::JThrowable};

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
    let is_cancelled = env.is_instance_of(exception, "one.tesseract.UserCancelledException")?;
    let message = env.call_method(exception, "getMessage", "()Ljava/lang/String;", &[])?.l()?;
    let message: String = env.get_string(message.into())?.into();

    let kind = if is_cancelled {
        tesseract::ErrorKind::Cancelled
    } else {
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
            let tesseract_error = tesseractify(env, || {
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
