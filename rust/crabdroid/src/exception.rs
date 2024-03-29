//===------------ exception.rs --------------------------------------------===//
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

use jni::descriptors::Desc;
use jni::errors::Result;
use jni::objects::{JObject, JThrowable};
use jni::JNIEnv;

/// Lifetime'd representation of a `TransceiverException`. Just a `JObject` wrapped in a
/// new class.
#[derive(Clone, Copy)]
pub struct Exception<'a: 'b, 'b> {
    internal: JObject<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> ::std::ops::Deref for Exception<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a: 'b, 'b> From<Exception<'a, 'b>> for JObject<'a> {
    fn from(other: Exception<'a, 'b>) -> JObject<'a> {
        other.internal
    }
}

impl <'a: 'b, 'b> Into<JThrowable<'a>> for Exception<'a, 'b> {
    fn into(self) -> JThrowable<'a> {
        let jobject: JObject = self.into();
        JThrowable::from(jobject)
    }
}

impl <'a: 'b, 'b> Desc<'a, JThrowable<'a>> for Exception<'a, 'b> {
    fn lookup(self, _: &JNIEnv<'a>) -> Result<JThrowable<'a>> {
        Ok(self.into())
    }
}

impl<'a: 'b, 'b> Exception<'a, 'b> {
    pub fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> Exception<'a, 'b> {
        Exception {
            internal: obj,
            env: env,
        }
    }

    pub fn new(env: &'b JNIEnv<'a>, message: Option<&str>) -> Result<Exception<'a, 'b>> {
        let jexception = if let Some(message) = message {
            let jmessage = env.new_string(message)?;
            env.new_object("java/lang/Exception", "(Ljava/lang/String;)V", &[jmessage.into()])?
        } else {
            env.new_object("java/lang/Exception", "()V", &[])?
        };
        Ok(Self::from_env(env, jexception))
    }

    pub fn get_message(&self) -> Result<String> {
        let message = self
            .env
            .call_method(self.internal, "getMessage", "()Ljava/lang/String;", &[])?
            .l()?;

        Ok(self.env.get_string(message.into())?.into())
    }

    pub fn print_stack_trace(&self) -> Result<()> {
        self.env.call_method(self.internal, "printStackTrace", "()V", &[])?.v()
    }
}
