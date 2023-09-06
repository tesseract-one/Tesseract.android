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

use interop_android::{ContextedGlobal, JFuture};
use jni::JNIEnv;
use jni::objects::JObject;
use jni::errors::Result;

use super::core::RustCore;

pub(crate) struct UI {
    core: ContextedGlobal,
}

impl UI {
    pub(crate) fn with_core(env: &JNIEnv, core: JObject) -> Result<Self> {
        ContextedGlobal::from_local(env, core).map(|core| {
            UI {core: core}
        })
    }

    pub(crate) async fn request_user_confirmation(&self, transaction: &str) -> tesseract::Result<bool> {
        debug!("!!!Before UI call");
        let allow = self.core.do_in_context_rret(64, |env, core| {
            let core = RustCore::from_env(&env, core);
            let allow = core.request_user_confirmation(transaction);

            Ok(JFuture::from_stage_result(allow))
        }).map_err(|e| {tesseract::Error::nested(Box::new(e))})?.await;
        debug!("!!!UI returned");

        allow.and_then(|allow| {
            allow.do_in_context_rret(64, |env, jallow| {
                env.call_method(jallow, "booleanValue", "()Z", &[])?.z()
            })
        }).map_err(|error| {
            tesseract::Error::nested(Box::new(error))
        })
    }
}