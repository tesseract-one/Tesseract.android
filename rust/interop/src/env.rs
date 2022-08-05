//===------------ env.rs --------------------------------------------===//
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

use jni::errors::{Error, Result};
use jni::objects::{GlobalRef, JClass, JObject, JValue};
use jni::strings::JNIString;
use jni::JNIEnv;

pub trait AndroidEnv<'a> {
    ///Is useful for loading classes on Android in non-main thread
    /// as it attempts to use a class loader registered for the thread.
    ///
    /// If there is no registered class loader - standard find class is used.
    /// Can't be used with multiple VMs which is fine on android anyway.
    fn find_class_android<S>(&self, name: S) -> Result<JClass<'a>>
    where
        S: Into<JNIString>;

    /// provide a class loader to be used in threads created by rust. Should be called after attach_thread on VM
    fn set_thread_class_loader(loader: GlobalRef) -> Result<()>;

    /// same as 'with_local_frame', but return an arbitrary value
    fn with_local_frame_arbitrary<F, R>(&self, capacity: i32, f: F) -> Result<R>
    where
        F: FnOnce() -> Result<R>;
}

use std::cell::RefCell;
thread_local!(pub static LOADER: RefCell<Option<GlobalRef>> = RefCell::new(None));

impl<'a> AndroidEnv<'a> for JNIEnv<'a> {
    fn set_thread_class_loader(loader: GlobalRef) -> Result<()> {
        let loader = loader.clone();

        let replaced = LOADER.with(|f| f.borrow_mut().replace(loader));

        if replaced.is_none() {
            Ok(())
        } else {
            Err(Error::FieldAlreadySet(
                "Trying to set class loader for the same thread twice".to_owned(),
            ))
        }
    }

    fn find_class_android<S>(&self, name: S) -> Result<JClass<'a>>
    where
        S: Into<JNIString>,
    {
        let name = name.into();
        let loader = LOADER.with(|f| f.borrow().clone());

        match loader {
            Some(loader) => {
                let class_name = self.new_string(name).expect("Couldn't create java string!");

                self.call_method(
                    JObject::from(loader.as_obj().into_inner()),
                    "loadClass",
                    "(Ljava/lang/String;)Ljava/lang/Class;",
                    &[JValue::from(class_name)],
                )?
                .l()
                .map(|o| o.into())
            }
            None => self.find_class(name),
        }
    }

    fn with_local_frame_arbitrary<F, R>(&self, capacity: i32, f: F) -> Result<R>
        where F: FnOnce() -> Result<R> {
        let mut result: Option<Result<R>> = None;
        
        let _ = self.with_local_frame(capacity, || {
            result = Some(f());

            Ok(JObject::null())
        })?;

        result.unwrap()
    }
}
