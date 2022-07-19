//===------------ response.rs --------------------------------------------===//
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

use jni::errors::{Error, Result};
use jni::objects::JObject;
use jni::JNIEnv;

use interop_android::env::AndroidEnv;
use interop_android::Exception;

pub enum Response {
    Ok(Vec<u8>),
    Cancelled,
    Exception(String),
    JniError(Error),
}

impl Response {
    fn from_java_non_flat(env: &JNIEnv, jresponse: JObject) -> Result<Self> {
        //let clazz_error = env.find_class_android("one/tesseract/ipc/TransceiverResponseError")?;
        let clazz_ok = env.find_class_android("one/tesseract/ipc/TransceiverResponseOk")?;
        let clazz_canceled =
            env.find_class_android("one/tesseract/ipc/TransceiverResponseCanceled")?;

        if env.is_same_object(jresponse, JObject::null())? {
            Ok(Self::Exception("Response is java null".to_owned()))
        } else {
            if env.is_instance_of(jresponse, clazz_ok)? {
                let array = env
                    .call_method(jresponse, "getData", "()[B", &[])?
                    .l()?
                    .into_inner();
                let vec = env.convert_byte_array(array)?;
                Ok(Self::Ok(vec))
            } else if env.is_instance_of(jresponse, clazz_canceled)? {
                Ok(Self::Cancelled)
            } else {
                let exception = env
                    .call_method(
                        jresponse,
                        "getException",
                        "()Lone/tesseract/ipc/TransceiverException;",
                        &[],
                    )?
                    .l()?;

                let exception = Exception::from_env(&env, exception);

                let message = exception.get_message()?;

                Ok(Self::Exception(message))
            }
        }
    }

    pub fn from_java(env: &JNIEnv, jresponse: JObject) -> Self {
        let res = Self::from_java_non_flat(env, jresponse);

        res.unwrap_or_else(|e| Self::JniError(e))
    }
}

pub trait Flattener {
    fn flatten(self) -> Response;
}

impl Flattener for Result<Response> {
    fn flatten(self) -> Response {
        match self {
            Ok(response) => response,
            Err(error) => Response::JniError(error),
        }
    }
}
