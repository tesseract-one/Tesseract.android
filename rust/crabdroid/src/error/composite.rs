use std::fmt;

use jni::{JNIEnv, objects::JObject};

use super::exception::ExceptionConvertible;
use super::local::LocalError;
use super::global::GlobalError;

#[derive(Debug)]
pub enum CompositeError<E: std::error::Error> {
    Jni(jni::errors::Error),
    Other(E)
}

impl<E: std::error::Error> fmt::Display for CompositeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompositeError::Jni(e) => e.fmt(f),
            CompositeError::Other(e) => <E as fmt::Display>::fmt(e, f)
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for CompositeError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CompositeError::Jni(e) => Some(e),
            CompositeError::Other(e) => Some(e),
        }
    }
}

//COMPOSITE ERROR CONVERSIONS

pub auto trait CompositeErrorInclude {
}

impl<T> !CompositeErrorInclude for CompositeError<T> {
}

impl<EI, E> From<EI> for CompositeError<E>
where
    E: std::error::Error + From<EI>,
    EI: std::error::Error + CompositeErrorInclude,
 {
    fn from(value: EI) -> Self {
        Self::Other(E::from(value))
    }
}

impl !CompositeErrorInclude for jni::errors::Error {
}

impl<E> From<jni::errors::Error> for CompositeError<E>
where
    E: std::error::Error,
 {
    fn from(value: jni::errors::Error) -> Self {
        Self::Jni(value)
    }
}

//COMPOSITE ERROR IMPLS

impl<E> CompositeError<E>
where
    E: std::error::Error,
    E: From<GlobalError>
{
    pub fn flatten_java(self, env: &JNIEnv) -> E {
        match self {
            CompositeError::Jni(e) => {
                let local = LocalError::with_exceptions_checking(env, e);
                E::from(local.into_global(env))
            },
            CompositeError::Other(e) => e,
        }
    }
}

impl<E> ExceptionConvertible for CompositeError<E>
where
    E: std::error::Error,
    E: ExceptionConvertible
{
    fn to_exception<'a: 'b, 'b>(&self, env: &'b JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        match self {
            CompositeError::Jni(e) => e.to_exception(env),
            CompositeError::Other(e) => e.to_exception(env),
        }
    }
}