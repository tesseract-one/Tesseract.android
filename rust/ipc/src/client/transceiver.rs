//===------------ transceiver.rs --------------------------------------------===//
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

use jni::errors::Result;
use jni::objects::JObject;
use jni::objects::JValue;
use jni::JNIEnv;

use crabdroid::future::completion_stage::JCompletionStage;
use crabdroid::env::AndroidEnv;
use crabdroid::JFuture;

/// Lifetime'd representation of a `Transceiver`. Just a `JObject` wrapped in a
/// new class.
#[derive(Clone, Copy)]
pub struct Transceiver<'a: 'b, 'b> {
    internal: JObject<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> ::std::ops::Deref for Transceiver<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a: 'b, 'b> From<Transceiver<'a, 'b>> for JObject<'a> {
    fn from(other: Transceiver<'a, 'b>) -> JObject<'a> {
        other.internal
    }
}

impl<'a: 'b, 'b> Transceiver<'a, 'b> {
    pub fn new(env: &'b JNIEnv<'a>, application: JObject<'a>) -> Result<Self> {
        let clazz = env.find_class_android("one/tesseract/ipc/client/Transceiver")?;
        let transceiver = env.new_object(
            clazz,
            "(Landroid/app/Application;)V",
            &[JValue::from(application)],
        )?;

        Ok(Self::from_env(env, transceiver))
    }

    pub fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> Self {
        Self {
            env: env,
            internal: obj,
        }
    }

    pub fn ping(&self, protocol: &str) -> bool {
        fn _ping<'a: 'b, 'b>(
            env: &'b JNIEnv<'a>,
            transceiver: JObject<'a>,
            protocol: &str,
        ) -> Result<bool> {
            let jproto = env.new_string(protocol)?;

            let pinging = env
                .call_method(
                    transceiver,
                    "ping",
                    "(Ljava/lang/String;)Z",
                    &[jproto.into()],
                )?
                .z()?;

            Ok(pinging)
        }

        let result = _ping(self.env, self.internal, protocol);

        result.unwrap_or_else(|e| {
            debug!("Pinging wallet returned an error.\nDescription: {}", e);
            false
        })
    }

    pub fn transceive(&self, data: &[u8], protocol: &str) -> JFuture {
        fn _transceive<'a: 'b, 'b>(
            env: &'b JNIEnv<'a>,
            transceiver: JObject<'a>,
            data: &[u8], protocol: &str,
        ) -> Result<JCompletionStage<'a, 'b>> {
            let data = env.byte_array_from_slice(data)?;
            let data = unsafe {JObject::from_raw(data)};

            let jproto = env.new_string(protocol)?;

            let raw = env
                .call_method(
                    transceiver,
                    "transceive",
                    "([BLjava/lang/String;)Ljava/util/concurrent/CompletionStage;",
                    &[data.into(), jproto.into()],
                )?
                .l()?;
            Ok(JCompletionStage::from_env(env, raw))
        }

        let result = self.env.with_exceptions_check(|| {
            _transceive(self.env, self.internal, data, protocol)
        }).map_err(|e| e.into_global(&self.env));

        JFuture::from_stage_result(result)
    }
}
