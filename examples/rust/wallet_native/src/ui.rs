//===------------ ui.rs --------------------------------------------===//
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

use jni::JNIEnv;
use jni::objects::JObject;
use jni::errors::Result as JResult;

use crabdroid::ContextedGlobal;

use crate::error::WalletError;
use crate::core::RustCore;

pub(crate) struct UI {
    core: ContextedGlobal,
}

impl UI {
    pub(crate) fn with_core(env: &JNIEnv, core: JObject) -> JResult<Self> {
        ContextedGlobal::from_local(env, core).map(|core| {
            UI {core: core}
        })
    }

    pub(crate) async fn request_user_confirmation(&self, transaction: &str) -> Result<bool, WalletError> {
        debug!("!!!Before UI call");

        let allow = self.core.with_async_context(64, |env, core| {
            let core = RustCore::from_env(&env, core);
            core.request_user_confirmation(transaction)
        }).await?;

        debug!("!!!UI returned");

        Ok(allow.with_safe_context_rret(64, |env, jallow| {
            env.call_method(jallow, "booleanValue", "()Z", &[])?.z()
        })?)
    }
}