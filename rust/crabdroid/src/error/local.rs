use jni::{JNIEnv, objects::{JObject, JThrowable}};

use thiserror::Error;

use crate::ContextedGlobal;
use super::global::GlobalError;

#[derive(Debug, Error)]
pub enum LocalError<'a> {
    #[error("An exception occured. To know more convert the error properly")]
    Exception(JObject<'a>),

    #[error(transparent)]
    JniError(jni::errors::Error)
}

pub type LocalResult<'a, T> = Result<T, LocalError<'a>>;

fn retrieve_exception<'a: 'b, 'b>(env: &'b JNIEnv<'a>) -> jni::errors::Result<JThrowable<'a>> {
    let exception = env.exception_occurred()?;
    env.exception_clear()?;

    Ok(exception)
}

impl<'a> LocalError<'a> {
    pub fn into_global(self, env: &JNIEnv<'a>) -> GlobalError {
        match self {
            Self::JniError(e) => GlobalError::JniError(e),
            Self::Exception(e) => {
                match ContextedGlobal::from_local(env, e) {
                    Err(e) => {
                        debug!("Something is going pretty bad; couldn't process exception in JNI: {0}", &e);
                        GlobalError::JniError(e)
                    },
                    Ok(e) => GlobalError::Exception(e)
                }
            }
        }
    }

    pub fn with_exceptions_checking(env: &JNIEnv<'a>, error: jni::errors::Error) -> Self {
        match error {
            jni::errors::Error::JavaException => {
                let exception = retrieve_exception(env);
                match exception {
                    Err(e) => LocalError::JniError(e),
                    Ok(exception) => {
                        LocalError::Exception(exception.into())
                    }
                }
            },
            other => {
                LocalError::JniError(other)
            }
        }
    }

    pub fn without_exceptions_checking(error: jni::errors::Error) -> Self {
        LocalError::JniError(error)
    }
}