use std::default::Default;

use jni::JNIEnv;

use crate::env::AndroidEnv;

use super::exception::ExceptionConvertible;
use super::global::GlobalError;
use super::composite::CompositeError;

pub trait JavaErrorContext: ExceptionConvertible + std::error::Error {
    fn java_context<T, I>(env: &JNIEnv, fun: impl FnOnce() -> Result<I, CompositeError<Self>>) -> T
    where
        Self: Sized,
        T: Default,
        I: Into<T>,
    {
        match fun() {
            Err(err) => {
                match env.throw_error(&err) {
                    Ok(_) => T::default(),
                    Err(e) => {
                        let message = err.to_string();
                        debug!("Error '{}' occured, but couldn't be thrown as Exception because JNI returned: {}", message, e.to_string());
                        panic!("Error '{}' occured, but couldn't be thrown as Exception because JNI returned: {}", message, e.to_string())
                    },
                }
            }
            Ok(value) => value.into()
        }
    }
}

impl<E> JavaErrorContext for E where E: ExceptionConvertible + std::error::Error {
}

pub trait CompositeErrorContext: std::error::Error {
    fn composite_context<T>(env: &JNIEnv, fun: impl FnOnce() -> Result<T, CompositeError<Self>>) -> Result<T, Self>
    where
        Self: Sized,
        Self: From<GlobalError>,
    {
        fun()
            .map_err(|e| e.flatten_java(env))
    }

    fn composite_context2<T>(env: &JNIEnv, fun: impl FnOnce() -> Result<T, CompositeError<Self>>) -> Result<T, CompositeError<Self>>
    where
        Self: Sized,
        Self: From<GlobalError>,
    {
        fun()
    }
}

impl<E> CompositeErrorContext for E
where
    E: std::error::Error
{
}