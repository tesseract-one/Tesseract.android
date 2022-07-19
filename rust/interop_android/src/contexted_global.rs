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

    pub fn local_env(&self) -> Result<(JNIEnv, JObject)> {
        Ok((self.guard.0.get_env()?, self.guard.1.as_obj()))
    }
}
