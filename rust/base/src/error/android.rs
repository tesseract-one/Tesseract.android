use jni::{JNIEnv, objects::JObject, errors::Result};

use thiserror::Error;

use log::SetLoggerError;

use crabdroid::error::{ExceptionConvertible, CompositeError, CompositeErrorInclude, GlobalError};

use super::tesseract::global_error_to_tesseract;

use super::exception::*;

#[derive(Debug, Error)]
pub enum TesseractAndroidError {
    #[error(transparent)]
    Tesseract(#[from] tesseract::Error),

    #[error(transparent)]
    Logger(#[from] SetLoggerError),

    #[error(transparent)]
    Gllobal(#[from] GlobalError)
}

impl Into<tesseract::Error> for TesseractAndroidError {
    fn into(self) -> tesseract::Error {
        match self {
            TesseractAndroidError::Tesseract(e) => e,
            TesseractAndroidError::Logger(e) => tesseract::Error::described(
                tesseract::ErrorKind::Weird,
                &e.to_string()),
            TesseractAndroidError::Gllobal(e) => global_error_to_tesseract(e),
        }
    }
}

impl ExceptionConvertible for TesseractAndroidError {
    fn to_exception<'a: 'b, 'b>(&self, env: &'b JNIEnv<'a>) -> Result<JObject<'a>> {
        match self {
            TesseractAndroidError::Tesseract(error) => tesseract_error_to_exception(error, env),
            TesseractAndroidError::Logger(error) => logger_error_to_exception(error, env),
            TesseractAndroidError::Gllobal(error) => global_error_to_exception(error, env),
        }
    }
}

impl !CompositeErrorInclude for TesseractAndroidError {
}

impl From<TesseractAndroidError> for CompositeError<TesseractAndroidError> {
    fn from(value: TesseractAndroidError) -> Self {
        Self::Other(value)
    }
}
