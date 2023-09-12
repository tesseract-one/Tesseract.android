//===------------ contexted_global.rs --------------------------------------------===//
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

use std::sync::Arc;

use jni::errors::Result;
use jni::objects::GlobalRef;
use jni::objects::JObject;
use jni::JNIEnv;
use jni::JavaVM;

use crate::JFuture;
use crate::env::AndroidEnv;
use crate::error::GlobalError;
use crate::error::GlobalResult;
use crate::error::LocalError;
use crate::error::LocalResult;
use crate::future::completion_stage::JCompletionStage;

/// GlobalRef along with it's VM. Mainly to be used for thread traveling of objects.
#[derive(Clone, Debug)]
pub struct ContextedGlobal {
    guard: Arc<(JavaVM, GlobalRef)>,
}

impl ContextedGlobal {
    pub fn from_local(env: &JNIEnv, local: JObject) -> Result<Self> {
        Ok(Self {
            guard: Arc::new((env.get_java_vm()?, env.new_global_ref(local)?)),
        })
    }

    fn local_env<'a>(&'a self) -> Result<(JNIEnv, JObject<'a>)> {
        Ok((self.guard.0.get_env()?, self.guard.1.as_obj()))
    }

    pub fn do_in_context_jret<'a: 'b, 'b, F>(&'a self, capacity: i32, f: F) -> Result<ContextedGlobal>
        where F: 'b + FnOnce(JNIEnv<'b>, JObject<'b>) -> Result<JObject<'b>>,
    {
        let (env, object) = self.local_env()?;
        let local = env.with_local_frame(capacity, || {
            f(env, object)
        })?;

        Self::from_local(&env, local)
    }

    pub fn do_in_context_rret<F, R>(&self, capacity: i32, f: F) -> Result<R>
        where F: FnOnce(JNIEnv, JObject) -> Result<R>
    {
        let (env, object) = self.local_env()?;
        env.with_local_frame_arbitrary(capacity, || {
            f(env, object)
        })
    }

    pub fn with_safe_context_rret<F, R>(&self, capacity: i32, f: F) -> GlobalResult<R>
        where F: FnOnce(&JNIEnv, JObject) -> Result<R>
    {
        let (env, object) = self.local_env().map_err(|e| GlobalError::JniError(e))?;

        env.with_exceptions_check(|| {
            env.with_local_frame_arbitrary(capacity, || {
                f(&env, object)
            })
        }).map_err(|e| {
            e.into_global(&env)
        })
    }

    pub async fn with_async_context<F>(&self, capacity: i32, f: F) -> GlobalResult<ContextedGlobal>
        where for<'a, 'b> F: FnOnce(&'b JNIEnv<'a>, JObject<'a>) -> Result<JCompletionStage<'a, 'b>> {
            let jfuture = {
                let (env, object) = self.local_env().map_err(|e| GlobalError::JniError(e))?;
    
                let result = env.with_exceptions_check(|| {
                    env.with_local_frame(capacity, || {
                        let stage = f(&env, object)?;
                        Ok(stage.into())
                    })
                })
                .map(|object| JCompletionStage::from_env(&env, object))
                .map_err(|e| {
                    e.into_global(&env)
                });
                JFuture::from_stage_result(result)
            };  

            jfuture.await
    }
}


// pub trait JavaInto<'a: 'b, 'b, T> {
//     fn into(self, env: &'b JNIEnv<'a>) -> T;
// }

// // impl<'a: 'b, 'b, S, T> JavaInto<'a, 'b, T> for S where S: Into<T> {
// //     fn into(self, _: &'b JNIEnv<'a>) -> T {
// //         Self::into(self)
// //     }
// // }

// impl<'a: 'b, 'b, T> JavaInto<'a, 'b, T> for T {
//     fn into(self, _: &'b JNIEnv<'a>) -> T {
//         self
//     }
// }

// impl<'a: 'b, 'b, S> JavaInto<'a, 'b, GlobalRef> for S where S: JavaInto<'a, 'b, JObject<'a>> {
//     fn into(self, env: &'b JNIEnv<'a>) -> GlobalRef {
//         todo!()
//     }
// }

// impl<'a: 'b, 'b, S> JavaInto<'a, 'b, ContextedGlobal> for S where S: JavaInto<'a, 'b, GlobalRef> {
//     fn into(self, env: &'b JNIEnv<'a>) -> ContextedGlobal {
//         todo!()
//     }
// }

// impl<'a: 'b, 'b> JavaInto<'a, 'b, String> for JObject<'a> {
//     fn into(self, env: &'b JNIEnv<'a>) -> String {
//         todo!()
//     }
// }

// impl<'a: 'b, 'b, T> JavaInto<'a, 'b, T> for T {
//     fn into(self, env: &'b JNIEnv<'a>) -> T {
//         todo!()
//     }
// }
