//===------------ completable_future.rs --------------------------------------------===//
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

use std::fmt::Display;

use jni::errors::Result;
use jni::objects::{JObject, JThrowable, GlobalRef};
use jni::JNIEnv;

use crate::env::AndroidEnv;
use crate::contexted_global::ContextedGlobal;

use super::completion_stage::JCompletionStage;

/// Lifetime'd representation of a `CompletableFuture`. Just a `JObject` wrapped in a
/// new class.
#[derive(Clone, Copy)]
pub struct JCompletableFuture<'a: 'b, 'b> {
    internal: JObject<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> ::std::ops::Deref for JCompletableFuture<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a: 'b, 'b> From<JCompletableFuture<'a, 'b>> for JObject<'a> {
    fn from(other: JCompletableFuture<'a, 'b>) -> JObject<'a> {
        other.internal
    }
}

impl<'a: 'b, 'b> From<JCompletableFuture<'a, 'b>> for ContextedGlobal {
    fn from(other: JCompletableFuture<'a, 'b>) -> ContextedGlobal {
        ContextedGlobal::from_local(other.env, other.internal)
            .expect("If developer is doing things properly it should work") //Better ideas than expect?
    }
}

impl<'a: 'b, 'b> From<JCompletableFuture<'a, 'b>> for JCompletionStage<'a, 'b> {
    fn from(other: JCompletableFuture<'a, 'b>) -> JCompletionStage<'a, 'b> {
        JCompletionStage::from_env(other.env, other.internal)
    }
}

impl<'a: 'b, 'b> JCompletableFuture<'a, 'b> {
    pub fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> JCompletableFuture<'a, 'b> {
        JCompletableFuture {
            internal: obj,
            env: env,
        }
    }

    ///new non completed
    pub fn new(env: &'b JNIEnv<'a>) -> Result<JCompletableFuture<'a, 'b>> {
        let clazz = env
            .find_class_android("java/util/concurrent/CompletableFuture")?;

        let future = env
            .new_object(clazz, "()V", &[])?;

        Ok(Self::from_env(env, future))
    }

    ///.complete
    pub fn success(&self, value: JObject) -> Result<bool> {
        self.env.call_method(self.internal,
            "complete", "(Ljava/lang/Object;)Z", &[value.into()])?.z()
    }

    //.completeExceptionally
    pub fn failure(&self, throwable: JThrowable) -> Result<bool> {
        self.env.call_method(self.internal,
            "completeExceptionally", "(Ljava/lang/Throwable;)Z", &[throwable.into()])?.z()
    }

    pub fn resolve(&self, result: std::result::Result<JObject, JThrowable>) -> Result<bool> {
        match result {
            Ok(ok) => self.success(ok),
            Err(err) => self.failure(err),
        }
    }

    pub fn resolve2(&self, result: std::result::Result<GlobalRef, JThrowable>) -> Result<bool> {
        match result {
            Ok(ok) => self.success(ok.as_obj()),
            Err(err) => self.failure(err),
        }
    }

    pub fn resolve3<E: Display>(&self, result: std::result::Result<GlobalRef, E>) -> Result<bool> {
        match result {
            Ok(ok) => self.success(ok.as_obj()),
            Err(err) => {
                self.failure(JThrowable::from_error(self.env, err))
            }
        }
    }
}

trait ConvertThrowable<'a>: Sized {
    fn from_error<E: Display>(env: &'a JNIEnv, e: E) -> Self;
}

impl<'a> ConvertThrowable<'a> for JThrowable<'a> {
    fn from_error<E: Display>(env: &'a JNIEnv, e: E) -> Self {
        let clazz = env
            .find_class_android("java/lang/Exception").unwrap();

        let message = format!("Rust error: {}", e);
        let message = env.new_string(message).unwrap();

        let exception = env
            .new_object(clazz, "(Ljava/lang/String;)V", &[message.into()]).unwrap();

        exception.into()
    }
}