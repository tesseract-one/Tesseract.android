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

#[macro_use]
extern crate log;
extern crate android_log;

mod core;
mod delegate;
mod application;

use std::sync::Arc;

use futures::Future;
use futures::future::FutureExt;
use futures::executor::ThreadPoolBuilder;

use jni::{JNIEnv, JavaVM};
use jni::objects::{GlobalRef, JObject, JString};
use jni::errors::Result;

use jni_fn::jni_fn;

use interop_android::JFuture;
use interop_android::future::completion_stage::JCompletionStage;
use interop_android::future::into_java::FutureJava;
use interop_android::thread_pool::AndroidThreadPoolBuilder;

use tesseract::client::{Service, Tesseract};

use tesseract_ipc_android::client::TransportIPCAndroid;

use tesseract_protocol_test::{Test, TestService};

use crate::application::Application;
use crate::core::RustCore;
use crate::delegate::TransportDelegate;

#[jni_fn("one.tesseract.example.app.RustCore")]
pub fn rustInit<'a>(env: JNIEnv<'a>, core: JObject<'a>, loader: JObject<'a>) {
    android_log::init("MyApp").unwrap();

    fn init_res<'a>(env: JNIEnv<'a>, core: JObject<'a>, loader: JObject<'a>) -> Result<()> {
        let core = RustCore::from_env(&env, core);

        let application = core.get_application()?;

        let tesseract = Tesseract::new(
                TransportDelegate::arc(
                    Application::from_env(&env, application)?))
            .transport(TransportIPCAndroid::new(&env, application));

        let service: Arc<dyn Service<Protocol = Test>> =
            tesseract.service(Test::Protocol);

        core.set_service(service)?;

        let loader = env.new_global_ref(loader)?;

        let vm = env.get_java_vm()?;

        let tp = ThreadPoolBuilder::new()
            .jvm(vm, Some(loader))
            //.pool_size(1)
            .create()
            .expect("Can't create ThreadPool");

        core.set_executor(tp)
    }

    match init_res(env, core, loader) {
        Ok(_) => {
            debug!("!!!!!@@@@@####init_res was called without an accident");
        }
        Err(e) => {
            debug!("!!!!!@@@@@####init_res created the following error: {}", e);
        }
    }
}

#[jni_fn("one.tesseract.example.app.RustCore")]
pub fn sign<'a>(env: JNIEnv<'a>, rcore: JObject<'a>, transaction: JString<'a>) -> JObject<'a> {
    fn makeTransaction_res<'a: 'b, 'b>(env: &'b JNIEnv<'a>, rcore: JObject<'a>, transaction: JString<'a>) -> Result<impl Future<Output = tesseract::Result<GlobalRef>>> {
        let core = RustCore::from_env(env, rcore);

        let transaction: String = env
            .get_string(transaction)?
            .into();

        let service = core.get_service()?;

        let vm = env.get_java_vm()?;
        let transaction = async move {
            service.sign_transaction(&transaction).await
        }.map(|x| {
            x.and_then(|signed| {
                fn convert(vm: JavaVM, str: String) -> jni::errors::Result<GlobalRef> {
                    let env = vm.get_env()?;
                    let jstr = env.new_string(str)?;
                    env.new_global_ref(jstr)
                }

                convert(vm, signed).map_err(|e| {
                    tesseract::Error::nested(Box::new(e))
                })
            })
        });

        return Ok(transaction);
    }

    let transaction = match makeTransaction_res(&env, rcore, transaction) {
        Ok(transaction) => {
            debug!("!!!!!@@@@@####makeTransaction was called without an accident");
            transaction.into_java(&env)
        }
        Err(e) => {
            debug!(
                "!!!!!@@@@@####makeTransaction created the following error: {}",
                e
            );
            async {
                Err(tesseract::Error::nested(Box::new(e)))
            }.into_java(&env)
            
        }
    };

    debug!("!!!!!!!!!!DONE!!!!!!!!!");

    transaction.into()
}

#[jni_fn("one.tesseract.example.app.RustCore")]
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