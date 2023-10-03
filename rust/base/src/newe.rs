use jni::{JNIEnv, objects::JObject, errors::Result};

use thiserror::Error;

use log::SetLoggerError;

use interop_android::error::{ExceptionConvertible, CompositeError, CompositeErrorInclude, GlobalError};

#[derive(Debug, Error)]
pub enum TesseractAndroidError {
    #[error(transparent)]
    Tesseract(#[from] tesseract::Error),

    #[error(transparent)]
    Logger(#[from] SetLoggerError),

    // #[error(transparent)]
    // Jni(#[from] jni::errors::Error),

    #[error(transparent)]
    Gllobal(#[from] GlobalError)
}

impl Into<tesseract::Error> for TesseractAndroidError {
    fn into(self) -> tesseract::Error {
        todo!()
    }
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
            //TesseractAndroidError::Jni(error) => error.to_exception(env),
            TesseractAndroidError::Gllobal(_) => todo!(),
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

// fn test() -> std::result::Result<(), CompositeError<TesseractAndroidError>> {
//     android_log::init("TESSERACTNNNN")/*.map_err(TesseractAndroidError::from)*/?;
//     let a: () = std::result::Result::Err(TesseractAndroidError::Jni(jni::errors::Error::JavaException))?;
//     android_log::init("TESSERACTNNNN").map_err(TesseractAndroidError::from)?;
//     panic!()
// }

// trait TesseractConvertibleError {
//     fn to_tesseract<'a: 'b, 'b>(&self, env: &'b JNIEnv<'a>) -> tesseract::Error;
// }

/*#[derive(Debug)]
enum ContextedError<E: std::error::Error> {
    Jni(jni::errors::Error),
    Other(E)
}

use std::{fmt, error::Error};

impl<E: std::error::Error> fmt::Display for ContextedError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContextedError::Jni(e) => e.fmt(f),
            ContextedError::Other(e) => <E as fmt::Display>::fmt(e, f)
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for ContextedError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ContextedError::Jni(e) => Some(e),
            ContextedError::Other(e) => Some(e),
        }
    }
}

impl<E> From<E> for ContextedError<E> where E: Error {
    fn from(value: E) -> Self {
        todo!()
    }
}

trait TesseractAndroidErrorCompatError {
    fn into(self) -> TesseractAndroidError;
}

// impl<E> TesseractAndroidErrorCompatError for E where E: Into<TesseractAndroidError> {
    
// }

impl<EI> From<EI> for ContextedError<TesseractAndroidError> where EI: TesseractAndroidErrorCompatError {
    fn from(value: EI) -> Self {
        todo!()
    }
}

// impl<E, EI> From<EI> for ContextedError<E> where EI: Into<E>, E: std::error::Error {
//     fn from(value: EI) -> Self {
//         todo!()
//     }
// }

// impl<E, EI> From<EI> for ContextedError<E> where EI: TesseractAndroidErrorCompatError, E: std::error::Error {
//     fn from(value: EI) -> Self {
//         todo!()
//     }
// }

// impl<E, EI> From<EI> for ContextedError<E> where E: Error, EI: Error, E: From<EI> {
//     fn from(value: EI) -> Self {
//         todo!()
//     }
// }

fn test() -> std::result::Result<(), ContextedError<TesseractAndroidError>> {
    android_log::init("TESSERACTNNNN")/*.map_err(TesseractAndroidError::from)*/?;
    panic!()
}

// trait MyFrom {
// }

// trait CompaError<E> {

// }

// // impl<E, EI> CompaError for EI where E: From<EI> {
    
// // }

// impl<E: std::error::Error, EI: std::error::Error> From<EI> for ContextedError<E> where Self == EI {
//     fn from(value: EI) -> Self {
//         todo!()
//     }
// }

// impl<E, EI> Llalala<E> for EI where E: From<EI> {
// }

// trait Llalala {
// }

// impl<E: std::error::Error, EI: std::error::Error> From<EI> for ContextedError<E> where EI: Llalala {
//     fn from(value: EI) -> Self {
//         todo!()
//     }
// }

// impl<E: std::error::Error, EI: std::error::Error> From<EI> for ContextedError<E> where EI: Into<E> {
//     fn from(value: EI) -> Self {
//         todo!()
//     }
// }
*/