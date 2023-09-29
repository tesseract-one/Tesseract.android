use jni::{JNIEnv, objects::JObject, errors::Result};

use thiserror::Error;

use log::SetLoggerError;

use interop_android::error::ExceptionConvertible;

#[derive(Debug, Error)]
pub enum TesseractAndroidError {
    #[error(transparent)]
    Tesseract(#[from] tesseract::Error),

    #[error(transparent)]
    Logger(#[from] SetLoggerError),

    #[error(transparent)]
    Jni(#[from] jni::errors::Error)
}

fn logger_error_to_exception<'a: 'b, 'b>(error: &SetLoggerError, env: &'b JNIEnv<'a>) -> Result<JObject<'a>> {
    let description = error.to_string();
    let description = format!("Can't set android logger for Tesseract: {}", description);
    let description = env.new_string(description)?;

    env.new_object(
        "java/lang/Exception",
        "(Ljava/lang/String;)V",
        &[description.into()])
}

fn tesseract_error_to_exception<'a: 'b, 'b>(error: &tesseract::Error, env: &'b JNIEnv<'a>) -> Result<JObject<'a>> {
    match &error.kind {
        tesseract::ErrorKind::Cancelled => {
            if let Some(description) = &error.description {
                let description = env.new_string(description)?;
                env.new_object(
                    "one/tesseract/exception/UserCancelledException",
                    "(Ljava/lang/String;)V",
                    &[description.into()])
            } else {
                env.new_object(
                    "one/tesseract/exception/UserCancelledException",
                    "()V",
                    &[])
            }
        },
        kind => {
            let kind = kind.to_string();

            let description = if let Some(description) = &error.description {
                format!("Tesseract error with of type '{}': {}", kind, description)
            } else {
                format!("Tesseract error with no description of type: : {}", kind)
            };

            let description = env.new_string(description)?;

            env.new_object(
                "java/lang/Exception",
                "(Ljava/lang/String;)V",
                &[description.into()])
        },
    }
}

impl ExceptionConvertible for TesseractAndroidError {
    fn to_exception<'a: 'b, 'b>(&self, env: &'b JNIEnv<'a>) -> Result<JObject<'a>> {
        match self {
            TesseractAndroidError::Tesseract(error) => tesseract_error_to_exception(error, env),
            TesseractAndroidError::Logger(error) => logger_error_to_exception(error, env),
            TesseractAndroidError::Jni(error) => error.to_exception(env),
        }
    }
}