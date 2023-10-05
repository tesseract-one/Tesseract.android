use thiserror::Error;

use crate::ContextedGlobal;

#[derive(Debug, Error)]
pub enum GlobalError {
    #[error("An exception occured. To know more convert the error properly")]
    Exception(ContextedGlobal),

    #[error(transparent)]
    JniError(jni::errors::Error)
}

pub type GlobalResult<T> = Result<T, GlobalError>;