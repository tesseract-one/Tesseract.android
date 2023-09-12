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

use crate::bi_consumer::RBiConsumer;
use crate::contexted_global::ContextedGlobal;
use crate::error::LocalResult;
use jni::errors::Result;
use jni::objects::JObject;
use jni::JNIEnv;

/// Lifetime'd representation of a `CompletableFuture`. Just a `JObject` wrapped in a
/// new class.
#[derive(Clone, Copy)]
pub struct JCompletionStage<'a: 'b, 'b> {
    internal: JObject<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> ::std::ops::Deref for JCompletionStage<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a: 'b, 'b> From<JCompletionStage<'a, 'b>> for JObject<'a> {
    fn from(other: JCompletionStage<'a, 'b>) -> JObject<'a> {
        other.internal
    }
}

impl<'a: 'b, 'b> From<JCompletionStage<'a, 'b>> for ContextedGlobal {
    fn from(other: JCompletionStage<'a, 'b>) -> ContextedGlobal {
        ContextedGlobal::from_local(other.env, other.internal)
            .expect("If developer is doing things properly it should work") //Better ideas than expect?
    }
}

impl<'a: 'b, 'b> JCompletionStage<'a, 'b> {
    pub fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> JCompletionStage<'a, 'b> {
        JCompletionStage {
            internal: obj,
            env: env,
        }
    }

    pub fn when_complete_consumer(
        self,
        consumer: RBiConsumer<'a, 'b>,
    ) -> Result<JCompletionStage<'a, 'b>> {
        let r = self.env.call_method(
            self.internal,
            "whenComplete",
            "(Ljava/util/function/BiConsumer;)Ljava/util/concurrent/CompletionStage;",
            &[consumer.into()],
        )?;

        Ok(JCompletionStage::from_env(self.env, r.l()?))
    }

    pub fn when_complete_legacy<'o, F: FnMut(JNIEnv, Result<JObject<'o>>) + Send + 'static>(
        self,
        mut f: F,
    ) -> Result<JCompletionStage<'a, 'b>> {
        self.when_complete_consumer(RBiConsumer::new(self.env, move |env, success, failure| {
            fn pick<'a>(
                env: &JNIEnv,
                success: JObject<'a>,
                failure: JObject,
            ) -> Result<JObject<'a>> {
                if env.is_same_object(failure, JObject::null())? {
                    Ok(success)
                } else {
                    //TODO: proper error handling with the actual exception passing
                    Err(jni::errors::Error::JavaException)
                }
            }

            f(env, pick(&env, success, failure))
        })?)
    }

    pub fn when_complete<'o, F: FnMut(JNIEnv, LocalResult<'o, JObject<'o>>) + Send + 'static>(
        self,
        mut f: F,
    ) -> Result<JCompletionStage<'a, 'b>> {
        self.when_complete_consumer(RBiConsumer::new(self.env, move |env, success, failure| {
            fn pick<'a>(
                env: &JNIEnv,
                success: JObject<'a>,
                failure: JObject<'a>,
            ) -> LocalResult<'a, JObject<'a>> {
                if env.is_same_object(failure, JObject::null())? {
                    Ok(success)
                } else {
                    Err(crate::error::LocalError::Exception(failure))
                }
            }

            f(env, pick(&env, success, failure))
        })?)
    }
}

// use std::pin::Pin;
// use std::task::{Context, Poll};

// impl<'a: 'b, 'b> std::future::Future for JCompletionStage<'a, 'b> {
//     type Output = Result<JObject<'a>>;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//     }
// }
