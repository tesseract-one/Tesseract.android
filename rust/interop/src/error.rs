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

use crate::ContextedGlobal;

pub fn deresultify<T, I, F>(env: &JNIEnv, fun: F) -> T
where
    T: Default,
    I: Into<T>,
    F: FnOnce() -> Result<I, Box<dyn Error>>,
{
    match fun() {
        Err(err) => {
            //temporary solution. need a proper conversion to Exception with a class
            let message: &str = &err.to_string();

            match env.throw(message) {
                Ok(_) => T::default(),
                Err(e) => {
                    debug!("Error '{}' occured, but couldn't be thrown as Exception because JNI returned: {}", message, e.to_string());
                    panic!("Error '{}' occured, but couldn't be thrown as Exception because JNI returned: {}", message, e.to_string())
                },
            }
        }
        Ok(value) => value.into()
    }
}

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