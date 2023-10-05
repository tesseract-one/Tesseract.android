//===------------ lib.rs --------------------------------------------===//
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

#![feature(async_closure)]

#[macro_use]
extern crate log;
extern crate android_log;

mod ui;
mod core;
mod service;
mod signature_provider;

use std::sync::Arc;

use crabdroid::error::JavaErrorContext;
use jni::objects::{JObject, JString};
use jni::JNIEnv;

use jni_fn::jni_fn;

use tesseract::service::Tesseract;
use tesseract_android::error::TesseractAndroidError;
use tesseract_android::service::transport::IPCTransport;

use crate::core::RustCore;
use crate::service::TestService;
use crate::ui::UI;
use crate::signature_provider::SignatureProvider;

#[jni_fn("one.tesseract.example.wallet.RustCore")]
pub fn rustInit(env: JNIEnv, core: JObject, data_dir: JString) {
    TesseractAndroidError::java_context(&env, || {
        android_log::init("DemoWalletRust")?;
        log_panics::Config::new()
            .backtrace_mode(log_panics::BacktraceMode::Unresolved)
            .install_panic_hook();

        let data_dir: String = env
            .get_string(data_dir)?
            .into();

        let ui = UI::with_core(&env, core)?;
        let signature_provider = Arc::new(SignatureProvider::new(&data_dir));

        debug!("!!!Before Tesseract");
        let tesseract = Tesseract::new()
            .transport(IPCTransport::default(&env)?)
            .service(TestService::new(ui, Arc::clone(&signature_provider)));
        debug!("!!!Tesseract initialized succesfully");
        let _ = Box::leak(Box::new(tesseract));//let's keep it alive. make a field later

        let core = RustCore::from_env(&env, core);
        core.set_signature_provider(signature_provider)?;

        Ok(())
    })
}

#[jni_fn("one.tesseract.example.wallet.RustCore")]
pub fn saveSignature(env: JNIEnv, core: JObject, signature: JString) {
    TesseractAndroidError::java_context(&env, || {
        let signature: String = env
            .get_string(signature)?
            .into();

        let core = RustCore::from_env(&env, core);
        let provider = core.get_signature_provider()?;
        provider.set_signature(&signature);

        Ok(())
    })
}

#[jni_fn("one.tesseract.example.wallet.RustCore")]
pub fn readSignature<'a>(env: JNIEnv<'a>, core: JObject<'a>) -> JString<'a> {
    TesseractAndroidError::java_context(&env, || {
        let core = RustCore::from_env(&env, core);
        let provider = core.get_signature_provider()?;
        let signature = provider.get_signature();
        Ok(env.new_string(&signature)?)
    })
}