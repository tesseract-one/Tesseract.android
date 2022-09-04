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


//WALLET PART BEGIN//
struct TestService {}

impl tesseract::service::Service for TestService {
    type Protocol = Test;

    fn protocol(&self) -> &Test {
        &Test::Protocol
    }

    fn to_executor(self) -> Box<dyn tesseract::service::Executor + Send + Sync> {
        Box::new(tesseract_protocol_test::service::TestExecutor::from_service(
            self,
        ))
    }
}

#[async_trait]
impl tesseract_protocol_test::TestService for TestService {
    async fn sign_transaction(self: Arc<Self>, req: &str) -> tesseract::Result<String> {
        if req == "make_error" {
            Err(Error::described(
                ErrorKind::Weird,
                "intentional error for test",
            ))
        } else {
            Ok(format!("{}_signed!", req))
        }
    }
}
//WALLET PART END//

#[jni_fn("one.tesseract.example.wallet.RustCore")]
pub fn rustInit(env: JNIEnv, core: JObject, loader: JObject) {
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

    debug!("!!!Before Tesseract");
    let tesseract = Tesseract::new().transport(Transport::default(&env).unwrap()).service(TestService{});
    debug!("!!!Tesseract initialized succesfully");
    let _ = Box::leak(Box::new(tesseract));//let's keep it alive. make a field later

    /*match init_res(env, core, loader) {
        Ok(_) => {
            debug!("!!!!!@@@@@####init_res was called without an accident");
        }
        Err(e) => {
            debug!("!!!!!@@@@@####init_res created the following error: {}", e);
        }
    }*/
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
