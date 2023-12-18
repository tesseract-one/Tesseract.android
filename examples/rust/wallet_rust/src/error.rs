use thiserror::Error;

use jni::{JNIEnv, objects::JObject, errors::Result};

use crabdroid::error::{ExceptionConvertible, CompositeErrorInclude, CompositeError};

use tesseract_android::error::TesseractAndroidError;

use std::io;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error(transparent)]
    TesseractAndroid(#[from] TesseractAndroidError),

    #[error(transparent)]
    IO(#[from] io::Error)
}

pub auto trait WalletErrorInclude {
}

impl !WalletErrorInclude for TesseractAndroidError {
}

impl<E> From<E> for WalletError
where
    E: Into<TesseractAndroidError> + WalletErrorInclude,
 {
    fn from(value: E) -> Self {
        Self::TesseractAndroid(value.into())
    }
}

impl Into<tesseract_one::Error> for WalletError {
    fn into(self) -> tesseract_one::Error {
        match self {
            WalletError::TesseractAndroid(e) => e.into(),
            WalletError::IO(e) => {
                let description = format!("IOError: {}", e);
                tesseract_one::Error::described(tesseract_one::ErrorKind::Weird, &description)
            }
        }
    }
}

impl ExceptionConvertible for WalletError {
    fn to_exception<'a: 'b, 'b>(&self, env: &'b JNIEnv<'a>) -> Result<JObject<'a>> {
        match self {
            WalletError::TesseractAndroid(e) => e.to_exception(env),
            WalletError::IO(e) => {
                let description = format!("IOError in Rust: {}", e);
                let description = env.new_string(description)?;

                env.new_object(
                    "java/lang/Exception",
                    "(Ljava/lang/String;)V",
                    &[description.into()])
            },
        }
    }
}

impl !CompositeErrorInclude for WalletError {
}

impl From<WalletError> for CompositeError<WalletError> {
    fn from(value: WalletError) -> Self {
        Self::Other(value)
    }
}