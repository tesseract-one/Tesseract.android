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

mod core;
mod delegate;
mod application;

use futures::{
    future::FutureExt,
    executor::ThreadPoolBuilder
};

use jni::JNIEnv;
use jni::objects::{JObject, JString};

use jni_fn::jni_fn;

use crabdroid::{
    future::JCompletionStage,
    thread_pool::AndroidThreadPoolBuilder,
    error::JavaErrorContext,
    JFuture
};

use tesseract::client::Tesseract;
use tesseract_protocol_test::{Test, TestService};

use tesseract_android::error::TesseractAndroidError;
use tesseract_android::client::transport::IPCTransport;

use crate::application::Application;
use crate::core::RustCore;
use crate::delegate::TransportDelegate;

#[jni_fn("one.tesseract.example.rust_app.RustCore")]
pub fn rustInit<'a>(env: JNIEnv<'a>, core: JObject<'a>, loader: JObject<'a>) {
    TesseractAndroidError::java_context(&env, || {
        let log_level = if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Error
        };
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log_level)
                .with_tag("RustDAppDemo"),
        );
        
        log_panics::Config::new()
            .backtrace_mode(log_panics::BacktraceMode::Resolved)
            .install_panic_hook();

        let core = RustCore::from_env(&env, core);

        let application = core.get_application()?;

        let tesseract = Tesseract::new(
                TransportDelegate::arc(
                    Application::from_env(&env, application)?))
            .transport(IPCTransport::new(&env, application)?);

        let service = tesseract.service(Test::Protocol);

        core.set_service(service)?;

        let loader = env.new_global_ref(loader)?;

        let vm = env.get_java_vm()?;

        let tp = ThreadPoolBuilder::new()
            .jvm(vm, Some(loader))
            //.pool_size(1)
            .create()
            .expect("Can't create ThreadPool");

        Ok(core.set_executor(tp)?)
    })
}

#[jni_fn("one.tesseract.example.rust_app.RustCore")]
pub fn sign<'a>(env: JNIEnv<'a>, rcore: JObject<'a>, transaction: JString<'a>) -> JObject<'a> {
    TesseractAndroidError::java_context(&env, || {
        let core = RustCore::from_env(&env, rcore);

        let transaction: String = env
            .get_string(transaction)?
            .into();

        let service = core.get_service()?;

        JCompletionStage::launch_async(&env, async move |vm| {
            let signed = service.sign_transaction(&transaction).await?;
    
            let env = vm.get_env()?;
            let signed = env.new_string(&signed)?;
    
            Ok(env.new_global_ref(signed)?)
        })
    })
}

#[jni_fn("one.tesseract.example.rust_app.RustCore")]
pub fn execute<'a>(env: JNIEnv<'a>, core: JObject<'a>, future: JObject<'a>) {
    let core = RustCore::from_env(&env, core);

    let stage = JCompletionStage::from_env(&env, future);
    let future = JFuture::from_stage(stage);

    let executor = core.get_executor().unwrap();

    executor.spawn_ok(future.map(|sig_java| {
        match sig_java {
            Ok(sig_global) => {
                sig_global.do_in_context_rret(64, |env, sig_local| {
                    let sig: String = env
                        .get_string(sig_local.into())
                        .expect("Couldn't get java string!")
                        .into();
                    Ok(debug!(
                        "!!!@@@###The executing futute returned a result: {}",
                        sig
                    ))
                })
            }
            Err(error) => {
                Ok(debug!("!!!@@@###The executing futute returned an error: {}", error))
            }
        }.unwrap()
    }));
}