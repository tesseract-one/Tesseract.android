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

use crate::env::AndroidEnv;

/// GlobalRef along with it's VM. Mainly to be used for thread traveling of objects.
#[derive(Clone)]
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
}
