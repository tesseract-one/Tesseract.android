//===------------ bi_consumer.rs --------------------------------------------===//
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

use jni::objects::{JObject, JValue};
use jni::JNIEnv;
use jni::errors::Result;

use jni_fn::jni_fn;

use super::env::AndroidEnv;

#[derive(Clone, Copy)]
pub struct RBiConsumer<'a: 'b, 'b> {
    internal: JObject<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> ::std::ops::Deref for RBiConsumer<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a: 'b, 'b> From<RBiConsumer<'a, 'b>> for JObject<'a> {
    fn from(other: RBiConsumer<'a, 'b>) -> JObject<'a> {
        other.internal
    }
}

impl<'a: 'b, 'b> RBiConsumer<'a, 'b> {
    //TODO: mark F with Send or something
    pub fn new<'o, F: FnMut(JNIEnv, JObject<'o>, JObject<'o>) + Send + 'static>(env: &'b JNIEnv<'a>, f: F) -> Result<Self> {
        let boxed: Box<dyn FnMut(JNIEnv, JObject<'o>, JObject<'o>)> = Box::new(f);
        let raw = Box::into_raw(Box::new(boxed));

        let long = raw as *const () as i64;

        let clazz = env
            .find_class_android("one/tesseract/crabdroid/RBiConsumer")?;

        let consumer = env
            .new_object(clazz, "(J)V", &[JValue::from(long)])?;

        Ok(RBiConsumer::from_env(&env, consumer))
    }

    fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> RBiConsumer<'a, 'b> {
        RBiConsumer {
            internal: obj,
            env: env,
        }
    }

    unsafe fn closure(&self) -> Result<Box<Box<dyn FnMut(JNIEnv, JObject, JObject)>>> {
        let long = self
            .env
            .get_field(self.internal, "closure", "J")?
            .j()?;

        let raw = long as *mut Box<dyn FnMut(JNIEnv, JObject, JObject)>;
        Ok(Box::from_raw(raw))
    }
}

#[jni_fn("one.tesseract.crabdroid.RBiConsumer")]
pub fn accept(env: JNIEnv, consumer: JObject, a: JObject, b: JObject) {
    debug!("!@#$BEFORE JUST START");
    let consumer = RBiConsumer::from_env(&env, consumer);
    let closure = unsafe { Box::leak(consumer.closure().unwrap()) };

    debug!("!@#$BEFORE JUST CLOSURE");

    closure(env, a, b);
}

#[jni_fn("one.tesseract.crabdroid.RBiConsumer")]
pub fn finalize(env: JNIEnv, consumer: JObject) {
    let consumer = RBiConsumer::from_env(&env, consumer);

    debug!("%%%%%%%drop biconsumer before");

    match unsafe { consumer.closure() } {
        Ok(consumer) => drop(consumer),
        Err(error) => debug!("Weird stuff just happened while trying to drop the consumer: {}", error),
    }
}
