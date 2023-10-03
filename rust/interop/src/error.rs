//===------------ error.rs --------------------------------------------===//
//  Copyright 2022, Tesseract Systems, Inc.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//===----------------------------------------------------------------------===//

use std::{default::Default, error::Error};

use jni::{JNIEnv, objects::{JObject, JThrowable}};

use thiserror::Error;

use crate::{ContextedGlobal, env::AndroidEnv};

pub trait ExceptionConvertible {
    fn to_exception<'a: 'b, 'b>(&self, env: &'b JNIEnv<'a>) -> jni::errors::Result<JObject<'a>>;
}

// pub trait Deresultify: ExceptionConvertible + std::fmt::Display {
//     fn deresultify<T, I, F>(env: &JNIEnv, fun: F) -> T
//     where
//         Self: Sized,
//         T: Default,
//         I: Into<T>,
//         F: FnOnce() -> Result<I, Self>,
//     {
//         match fun() {
//             Err(err) => {
//                 match env.throw_error(&err) {
//                     Ok(_) => T::default(),
//                     Err(e) => {
//                         let message = err.to_string();
//                         debug!("Error '{}' occured, but couldn't be thrown as Exception because JNI returned: {}", message, e.to_string());
//                         panic!("Error '{}' occured, but couldn't be thrown as Exception because JNI returned: {}", message, e.to_string())
//                     },
//                 }
//             }
//             Ok(value) => value.into()
//         }
//     }
// }

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

impl ExceptionConvertible for jni::errors::Error {
    fn to_exception<'a: 'b, 'b>(&self, env: &'b JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        match self {
            jni::errors::Error::JavaException => {
                let exception = env.exception_occurred()?;
                env.exception_clear()?; //let's see how it works. probably we should leave it here
                Ok(exception.into())
                //TODO: wrap into JNIException
            },
            e => {
                let message = e.to_string();
                let message = env.new_string(message)?;
                env.new_object(
                    "one/tesseract/interop/rust/InteropException",
                    "(Ljava/lang.String;)V",
                    &[message.into()])
            }
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

use std::fmt;

#[derive(Debug)]
pub enum CompositeError<E: std::error::Error> {
    Jni(jni::errors::Error),
    Other(E)
}

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


// pub fn deresultify<T, I, F>(env: &JNIEnv, fun: F) -> T
// where
//     T: Default,
//     I: Into<T>,
//     F: FnOnce() -> Result<I, Box<dyn Error>>,
// {
//     match fun() {
//         Err(err) => {
//             //temporary solution. need a proper conversion to Exception with a class
//             let message: &str = &err.to_string();

//             match env.throw(message) {
//                 Ok(_) => T::default(),
//                 Err(e) => {
//                     debug!("Error '{}' occured, but couldn't be thrown as Exception because JNI returned: {}", message, e.to_string());
//                     panic!("Error '{}' occured, but couldn't be thrown as Exception because JNI returned: {}", message, e.to_string())
//                 },
//             }
//         }
//         Ok(value) => value.into()
//     }
// }

#[derive(Debug, Error)]
pub enum LocalError<'a> {
    #[error("An exception occured. To know more convert the error properly")]
    Exception(JObject<'a>),

    #[error(transparent)]
    JniError(jni::errors::Error)
}

#[derive(Debug, Error)]
pub enum GlobalError {
    #[error("An exception occured. To know more convert the error properly")]
    Exception(ContextedGlobal),

    #[error(transparent)]
    JniError(jni::errors::Error)
}

pub type LocalResult<'a, T> = Result<T, LocalError<'a>>;
pub type GlobalResult<T> = Result<T, GlobalError>;

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

// pub trait ExceptionCheck<'local> {
//     fn with_exceptions_check<T, F>(&self, fun: F) -> LocalResult<'local, T>
//         where F: FnOnce(&Self) -> jni::errors::Result<T>;
// }

// impl<'local> ExceptionCheck<'local> for JNIEnv<'local> {
//     fn with_exceptions_check<T, F>(&self, fun: F) -> LocalResult<'local, T>
//         where F: FnOnce(&Self) -> jni::errors::Result<T> {
//             fun(self).map_err(|e| LocalError::with_exceptions_checking(self, e))
//         }
// }