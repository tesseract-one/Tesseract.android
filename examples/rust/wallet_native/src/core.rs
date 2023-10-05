//===------------ core.rs --------------------------------------------===//
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

use jni::{objects::{JObject, JValue}, JNIEnv};
use jni::errors::Result;

use crabdroid::{future::completion_stage::JCompletionStage, pointer::ArcPointer};

use crate::signature_provider::SignatureProvider;

/// Lifetime'd representation of a `RustCore`. Just a `JObject` wrapped in a
/// new class.
#[derive(Clone, Copy)]
pub struct RustCore<'a: 'b, 'b> {
    internal: JObject<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> ::std::ops::Deref for RustCore<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a: 'b, 'b> From<RustCore<'a, 'b>> for JObject<'a> {
    fn from(other: RustCore<'a, 'b>) -> JObject<'a> {
        other.internal
    }
}

impl<'a: 'b, 'b> RustCore<'a, 'b> {
    pub fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> RustCore<'a, 'b> {
        RustCore {
            internal: obj,
            env: env,
        }
    }

    pub (crate) fn request_user_confirmation(&self, transaction: &str) -> Result<JCompletionStage<'a, 'b>> {
        let transaction = self.env.new_string(transaction)?;

        let stage = self.env
            .call_method(
                self.internal,
                "requestUserConfirmation",
                "(Ljava/lang/String;)Ljava/util/concurrent/CompletionStage;",
                &[JValue::from(transaction)],
            )?
            .l()?;

        Ok(JCompletionStage::from_env(&self.env, stage))
    }

    pub (crate) fn get_signature_provider(&self) -> Result<Arc<SignatureProvider>> {
        let provider_l = self
            .env
            .call_method(self.internal, "getSignatureProvider", "()J", &[])?
            .j()?;

        Ok(ArcPointer::of(provider_l).arc())
    }

    pub (crate) fn set_signature_provider(&self, provider: Arc<SignatureProvider>) -> Result<()> {
        self.env
            .call_method(
                self.internal,
                "setSignatureProvider",
                "(J)V",
                &[JValue::Long(ArcPointer::new(provider).into())],
            )?
            .v()
    }
}