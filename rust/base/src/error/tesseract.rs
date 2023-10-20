use jni::{JNIEnv, objects::JThrowable};

use crabdroid::{error::GlobalError, env::AndroidEnv};

use log::debug;
use tesseract::Error as TError;
use jni::errors::Error as JError;

pub fn jni_error_to_tesseract(error: JError) -> TError {
    TError::new(
        tesseract::error::ErrorKind::Weird,
        &format!("JNI Error: {}", error.to_string()),
        error)
}

pub fn exception_to_tesseract<'a: 'b, 'b>(env: &'b JNIEnv<'a>, exception: JThrowable<'a>) -> jni::errors::Result<tesseract::Error> {
    let user_cancelled_clazz = env.find_class_android("one/tesseract/exception/UserCancelledException")?;
    let this_clazz = env.get_object_class(exception)?;
    let is_cancelled = env.is_assignable_from(user_cancelled_clazz, this_clazz)?;
    //is_instance_of(exception, clazz)?;
    let message = env.call_method(exception, "getMessage", "()Ljava/lang/String;", &[])?.l()?;
    let message: Option<String> = if message.is_null() {
        None
    } else {
        Some(env.get_string(message.into())?.into())
    };

    debug!("PRINTME {:#?}", &message);

    let kind = if is_cancelled {
        debug!("ITISCANCELLED");
        tesseract::ErrorKind::Cancelled
    } else {
        debug!("ITISWEIRD");
        tesseract::ErrorKind::Weird
    };

    Ok(if let Some(message) = message {
        TError::described(kind, &message)
    } else {
        TError::kinded(kind)
    })
}

pub fn global_error_to_tesseract(error: GlobalError) -> tesseract::Error {
    match error {
        GlobalError::Exception(e) => {
            let result = e.do_in_context_rret(64, |env, exception| {
                exception_to_tesseract(&env, exception.into())
            });
            match result {
                Ok(e) => e,
                Err(e) => jni_error_to_tesseract(e)
            }
        },
        GlobalError::JniError(e) => jni_error_to_tesseract(e)
    }
}