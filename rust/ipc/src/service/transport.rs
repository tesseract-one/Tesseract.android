//===------------ transport.rs --------------------------------------------===//
//  Copyright 2021, Tesseract Systems, Inc.
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

use interop_android::env::AndroidEnv;
use jni::JNIEnv;
use jni::JavaVM;
use jni::objects::GlobalRef;

use tesseract::service::BoundTransport;
use tesseract::service::Transport;
use tesseract::service::TransportProcessor;

use super::processor::JProcessor;

pub struct IPCTransport {
    channel: String,
    vm: JavaVM
}

impl IPCTransport {
    pub fn new<'a: 'b, 'b>(env: &'b JNIEnv<'a>, channel: &str) -> jni::errors::Result<Self> { //TODO: change to other error or implement status
        Ok(Self {
            channel: channel.to_owned(),
            vm: env.get_java_vm()?
        })
    }

    pub fn default<'a: 'b, 'b>(env: &'b JNIEnv<'a>) -> jni::errors::Result<Self> { //TODO: change to other error or implement status
        Self::new(env, "default")
    }
}

struct BoundIPCTransport {
    _channel: GlobalRef //keeps the channel alive
}

impl BoundTransport for BoundIPCTransport {}

impl Transport for IPCTransport {
    fn bind(self, processor: Arc<dyn TransportProcessor + Send + Sync>) -> Box<dyn BoundTransport> {
        debug!("!!!Binding");
        let env = self.vm.get_env().unwrap();
        debug!("!!!ENV");
        let processor = JProcessor::new(&env, processor).unwrap();
        debug!("!!!PROC");

        let channel = env.new_string(&self.channel).unwrap();
        debug!("!!!CHANNEL STRING");
        let clazz = env.find_class_android("one/tesseract/ipc/service/Channel").unwrap();
        debug!("!!!HAVE CHHANNEL CLASS");

        // let e = env.call_static_method(clazz, "create",
        //      "(Ljava/lang/String;Lone/tesseract/ipc/service/Processor;)Lone/tesseract/ipc/service/Channel;", 
        //      &[channel.into(), processor.into()]).err().unwrap();

        // debug!("?????E: {}", e);
        // panic!()

        // let channel = env.call_static_method(clazz, "create",
        //      "(Ljava/lang/String;Lone/tesseract/ipc/service/Processor;)Lone/tesseract/ipc/service/Channel;", 
        //      &[channel.into(), processor.into()]).unwrap().l().unwrap();
        // debug!("!!!GOT CHANNEL");

        let channel = env.call_method(*processor, "createChannel",
            "(Ljava/lang/String;)Lone/tesseract/ipc/service/Channel;", &[channel.into()]).unwrap().l().unwrap();
        debug!("!!!GOT CHANNEL");

        let channel = env.new_global_ref(channel).unwrap();

        debug!("!!!GLOBAL CHANNEL");

        Box::new(BoundIPCTransport {_channel: channel})
    }
}