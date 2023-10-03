use std::{error::Error, marker::PhantomData, fmt::Display};

use futures::Future;
use jni::{JNIEnv, objects::{JObject, GlobalRef}};

use crate::{error::{ExceptionConvertible, CompositeError}, env::AndroidEnv};

pub struct Context<E: std::error::Error> {
    _phantom: PhantomData<E>
}

impl<E: ExceptionConvertible + Display + std::error::Error> Context<E> {
   pub fn java_local<T, I, F>(env: &JNIEnv, fun: F) -> T
    where
        T: Default,
        I: Into<T>,
        F: FnOnce() -> Result<I, CompositeError<E>>,
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


