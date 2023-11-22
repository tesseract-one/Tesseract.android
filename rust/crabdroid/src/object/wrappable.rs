//===------------ wrappable.rs --------------------------------------------===//
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

use jni::objects::{JObject, JValue};
use jni::JNIEnv;
use jni::errors::{Error, Result};

use jni_fn::jni_fn;

use crate::env::AndroidEnv;
use crate::pointer::ArcPointer;
use crate::error::JavaErrorContext;

use super::desc::JavaDesc;

struct WrappableHandle {
    pointer: i64,
    dropper: Option<Box<dyn FnOnce(i64)>>
}

impl WrappableHandle {
    fn from_arc<T>(arc: Arc<T>) -> Self where T: ?Sized {
        let long_p: i64 = ArcPointer::new(arc).into();

        Self { pointer: long_p, dropper: Some(Box::new(|pointer| {
            let arc_pointer = ArcPointer::<T>::of(pointer);
            arc_pointer.destroy()
        }))}
    }

    fn from_java_ref(object: JObject, env: &JNIEnv) -> Result<Box<Self>> {
        let handle_lp = env
            .call_method(object, "getHandle", "()J", &[])?
            .j()?;

        let handle_p = handle_lp as *mut WrappableHandle;
        Ok(unsafe { Box::from_raw(handle_p) })
    }

    fn arc<T>(&self) -> Arc<T> where T: ?Sized {
        ArcPointer::of(self.pointer).arc()
    }
}

impl Drop for WrappableHandle {
    fn drop(&mut self) {
        let dropper = self.dropper.take();

        if let Some(dropper) = dropper {
            dropper(self.pointer)
        }
    }
}

pub struct JavaWrapper {
}

impl JavaWrapper {
    pub fn java_ref<'a: 'b, 'b, 'c, T>(arc: Arc<T>, clazz: &str, env: &'b JNIEnv<'a>) -> Result<JObject<'a>> where T: ?Sized {
        let clazz = env
            .find_class_android(clazz)?;

        let handle = WrappableHandle::from_arc(arc);
        let handle_p = Box::into_raw(Box::new(handle)) as *const () as i64;

        let obj = env.new_object(clazz, "(J)V", &[JValue::from(handle_p)])?;

        Ok(obj)
    }

    pub fn from_java_ref<T>(object: JObject, env: &JNIEnv) -> Result<Arc<T>> where T: ?Sized {
        let handle= Box::leak(WrappableHandle::from_java_ref(object, env)?);
        Ok(handle.arc())
    }
}

pub trait JavaWrappableDesc: JavaDesc {
}

pub trait JavaWrappable {
    fn java_ref<'a: 'b, 'b, D: JavaDesc>(self: Arc<Self>, env: &'b JNIEnv<'a>, desc: Option<D>) -> Result<JObject<'a>>;
    fn from_java_ref(object: JObject, env: &JNIEnv) -> Result<Arc<Self>>;
}

impl<T> JavaWrappable for T where T: JavaWrappableDesc {
    fn java_ref<'a: 'b, 'b, D: JavaDesc>(self: Arc<Self>, env: &'b JNIEnv<'a>, desc: Option<D>) -> Result<JObject<'a>> {
        let clazz = match &desc {
            Some(desc) => desc.java_class(),
            None => self.java_class(),
        };

        JavaWrapper::java_ref(Arc::clone(&self), clazz, env)
    }

    fn from_java_ref(object: JObject, env: &JNIEnv) -> Result<Arc<Self>> {
        JavaWrapper::from_java_ref(object, env)
    }
}

#[jni_fn("one.tesseract.crabdroid.RustObject")]
pub fn drop(env: JNIEnv, this: JObject) {
    Error::java_context(&env, || {
        let handle = WrappableHandle::from_java_ref(this, &env)?;
        Ok(drop(handle))
    })
}

