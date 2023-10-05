//===------------ thread_pool.rs --------------------------------------------===//
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

use futures::executor::ThreadPoolBuilder;
use jni::objects::GlobalRef;
use jni::{JNIEnv, JavaVM};

use super::env::AndroidEnv;

pub trait AndroidThreadPoolBuilder {
    fn jvm(&mut self, vm: JavaVM, class_loader: Option<GlobalRef>) -> &mut Self;
}

impl AndroidThreadPoolBuilder for ThreadPoolBuilder {
    fn jvm(&mut self, vm: JavaVM, class_loader: Option<GlobalRef>) -> &mut Self {
        self.after_start(move |_| {
            let _ = vm
                .attach_current_thread_permanently()
                .expect("Can't attach thread to VM in pool");

            debug!("THREAD POOL EXECUTOR THREAD IS INITIALIZED WITH JAVA");

            if let Some(loader) = class_loader.clone() {
                JNIEnv::set_thread_class_loader(loader).unwrap();
                debug!("THREAD POOL EXECUTOR THREAD IS SET WITH CLASS LOADER");
            }

            debug!("THREAD POOL EXECUTOR THREAD INITIALIZATION FINISHED WELL");
        })
    }
}
