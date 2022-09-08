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

//#![feature(proc_macro_is_available)]
//#![feature(once_cell)]

#[macro_use]
extern crate log;
extern crate android_log;

use std::sync::Arc;
use async_trait::async_trait;

use interop_android::{ContextedGlobal, JFuture};
use interop_android::future::completion_stage::JCompletionStage;
use jni::errors::Result;
use jni::objects::{GlobalRef, JClass, JObject, JString, JValue};
use jni::sys::{jbyteArray, jint, jlong, jstring};
use jni::JNIEnv;

use jni_fn::jni_fn;

use tesseract::Error;
use tesseract::ErrorKind;

use tesseract::service::Tesseract;
use tesseract_ipc_android::service::Transport;
use tesseract_protocol_test::Test;

use crate::core::RustCore;
use crate::service::TestService;
use crate::ui::UI;
use crate::signature_provider::SignatureProvider;

mod ui;
mod core;
mod service;
mod signature_provider;



#[jni_fn("one.tesseract.example.wallet.RustCore")]
pub fn rustInit(env: JNIEnv, core: JObject, data_dir: JString) {
    /*fn init_res(env: JNIEnv, core: JObject, loader: JObject) -> Result<()> {
        // let core = RustCore::from_env(&env, core);

        // let application = core.get_application()?;

        // let tesseract = Tesseract::new(SingleTransportDelegate::arc())
        //     .transport(TransportIPCAndroid::new(&env, application));

        // let service: Arc<dyn Service<Protocol = Polkadot>> =
        //     tesseract.service(Polkadot::Network);

        // core.set_service(service)?;

        // let loader = env.new_global_ref(loader)?;

        // let vm = env.get_java_vm()?;

        // let tp = ThreadPoolBuilder::new()
        //     .jvm(vm, Some(loader))
        //     .create()
        //     .expect("Can't create ThreadPool");

        //core.set_executor(tp)

        todo!()
    }*/

    android_log::init("MyApp").unwrap();

    let data_dir: String = env
            .get_string(data_dir).unwrap()
            .into();

    let ui = UI::with_core(&env, core).unwrap();
    let signature_provider = Arc::new(SignatureProvider::new(&data_dir));

    debug!("!!!Before Tesseract");
    let tesseract = Tesseract::new()
        .transport(Transport::default(&env).unwrap())
        .service(TestService::new(ui, Arc::clone(&signature_provider)));
    debug!("!!!Tesseract initialized succesfully");
    let _ = Box::leak(Box::new(tesseract));//let's keep it alive. make a field later

    let core = RustCore::from_env(&env, core);
    core.set_signature_provider(signature_provider).unwrap();

    /*match init_res(env, core, loader) {
        Ok(_) => {
            debug!("!!!!!@@@@@####init_res was called without an accident");
        }
        Err(e) => {
            debug!("!!!!!@@@@@####init_res created the following error: {}", e);
        }
    }*/
}

#[jni_fn("one.tesseract.example.wallet.RustCore")]
pub fn saveSignature(env: JNIEnv, core: JObject, signature: JString) {
    let signature: String = env
        .get_string(signature).unwrap()
        .into();

    let core = RustCore::from_env(&env, core);
    let provider = core.get_signature_provider().unwrap();
    provider.set_signature(&signature);
}