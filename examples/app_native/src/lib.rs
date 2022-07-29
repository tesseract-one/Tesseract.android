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

use jni::errors::Result;
use jni::objects::{GlobalRef, JClass, JObject, JString, JValue};
use jni::sys::{jbyteArray, jint, jlong, jstring};
use jni::JNIEnv;
use std::sync::Arc;
use tesseract::client::Service;
//use std::lazy::Lazy;
use std::sync::Mutex;
use std::{sync::mpsc, thread, time::Duration};

use jni_fn::jni_fn;

use interop_android::bi_consumer::RBiConsumer;
use interop_android::future::completion_stage::JCompletionStage;
use interop_android::JFuture;

use futures::executor::ThreadPool;
use futures::executor::ThreadPoolBuilder;
use futures::future::FutureExt;

use interop_android::env::AndroidEnv;
use interop_android::thread_pool::AndroidThreadPoolBuilder;

// #[macro_use]
// extern crate lazy_static;

// lazy_static! {
//     static ref tp: ThreadPool = ThreadPool::new().unwrap();
// }

//static _tp: Lazy<Mutex<ThreadPool>> = Lazy::new(|| Mutex::new(ThreadPool::new().unwrap()));

//fn tp() -> &ThreadPool {}

use tesseract::client::delegate::SingleTransportDelegate;
use tesseract::client::Tesseract;
use tesseract_ipc_android::client::TransportIPCAndroid;

use tesseract_protocol_test::Test;
use tesseract_protocol_test::TestService;

use interop_android::pointer::ArcPointer;

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
    fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> RustCore<'a, 'b> {
        RustCore {
            internal: obj,
            env: env,
        }
    }

    fn get_application(&self) -> Result<JObject> {
        self.env
            .call_method(
                self.internal,
                "getApplication",
                "()Lcom/example/clientapp/Application;",
                &[],
            )?
            .l()
    }

    fn get_service(&self) -> Result<Arc<dyn Service<Protocol = Test>>> {
        let service_l = self
            .env
            .call_method(self.internal, "getService", "()J", &[])?
            .j()?;

        Ok(ArcPointer::of(service_l).arc())
    }

    fn set_service(&self, service: Arc<dyn Service<Protocol = Test>>) -> Result<()> {
        self.env
            .call_method(
                self.internal,
                "setService",
                "(J)V",
                &[JValue::Long(ArcPointer::new(service).into())],
            )?
            .v()
    }

    fn get_executor(&self) -> Result<&mut ThreadPool> {
        let tpl = self
            .env
            .call_method(self.internal, "getExecutor", "()J", &[])?
            .j()?;

        let tpr = tpl as *mut ThreadPool;
        let tp = Box::leak(unsafe { Box::from_raw(tpr) });

        Ok(tp)
    }

    fn set_executor(&self, tp: ThreadPool) -> Result<()> {
        let tpl = Box::into_raw(Box::new(tp)) as *const () as i64;

        self.env
            .call_method(self.internal, "setExecutor", "(J)V", &[JValue::Long(tpl)])?
            .v()
    }
}

#[jni_fn("com.example.clientapp.RustCore")]
pub fn rustInit(env: JNIEnv, core: JObject, loader: JObject) {
    fn init_res(env: JNIEnv, core: JObject, loader: JObject) -> Result<()> {
        let core = RustCore::from_env(&env, core);

        let application = core.get_application()?;

        let tesseract = Tesseract::new(SingleTransportDelegate::arc())
            .transport(TransportIPCAndroid::new(&env, application));

        let service: Arc<dyn Service<Protocol = Test>> =
            tesseract.service(Test::Protocol);

        core.set_service(service)?;

        let loader = env.new_global_ref(loader)?;

        let vm = env.get_java_vm()?;

        let tp = ThreadPoolBuilder::new()
            .jvm(vm, Some(loader))
            .create()
            .expect("Can't create ThreadPool");

        core.set_executor(tp)
    }

    android_log::init("MyApp").unwrap();

    match init_res(env, core, loader) {
        Ok(_) => {
            debug!("!!!!!@@@@@####init_res was called without an accident");
        }
        Err(e) => {
            debug!("!!!!!@@@@@####init_res created the following error: {}", e);
        }
    }
}

#[jni_fn("com.example.clientapp.RustCore")]
pub fn makeTransaction(env: JNIEnv, rcore: JObject) {
    fn makeTransaction_res(env: JNIEnv, rcore: JObject) -> Result<()> {
        let vm = env.get_java_vm()?;
        let grcore = env.new_global_ref(rcore).unwrap();

        let core = RustCore::from_env(&env, rcore);

        let service = core.get_service()?;
        let tp = core.get_executor()?;

        let transaction = service.sign_transaction("TestTran");
        tp.spawn_ok(transaction.map(move |x| {
            let env = vm.get_env().unwrap();
            makeTransaction_res(env, grcore.as_obj());
            drop(grcore);
            match x {
            Ok(result) => {
                debug!(
                    "!!!!@@@######1The freaking transaction is finally signed1: {}",
                    result
                );
            }
            Err(error) => {
                debug!("!!!!@@@@#### for now I'm happy with the error: {}", error);
            }
        }}));

        Ok(())
    }

    match makeTransaction_res(env, rcore) {
        Ok(_) => {
            debug!("!!!!!@@@@@####makeTransaction was called without an accident");
        }
        Err(e) => {
            debug!(
                "!!!!!@@@@@####makeTransaction created the following error: {}",
                e
            );
        }
    }
}

#[jni_fn("com.example.clientapp.MainActivity")]
pub fn helloRust(
    env: JNIEnv,
    // this is the class that owns our
    // static method. Not going to be
    // used, but still needs to have
    // an argument slot
    _class: JClass,
    input: JString,
) -> jstring {
    // First, we have to get the string out of java. Check out the `strings`
    // module for more info on how this works.
    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();

    // Then we have to create a new java string to return. Again, more info
    // in the `strings` module.
    let output = env
        .new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");
    // Finally, extract the raw pointer to return.
    output.into_inner()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
