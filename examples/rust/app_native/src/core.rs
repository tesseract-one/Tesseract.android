use std::sync::Arc;

use futures::executor::ThreadPool;

use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use jni::errors::Result;

use crabdroid::pointer::ArcPointer;

use tesseract::client::Service;
use tesseract_protocol_test::Test;

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
    pub (crate) fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> RustCore<'a, 'b> {
        RustCore {
            internal: obj,
            env: env,
        }
    }

    pub (crate) fn get_application(&self) -> Result<JObject> {
        self.env
            .call_method(
                self.internal,
                "getApplication",
                "()Lone/tesseract/example/rust_app/Application;",
                &[],
            )?
            .l()
    }

    pub (crate) fn get_service(&self) -> Result<Arc<dyn Service<Protocol = Test>>> {
        let service_l = self
            .env
            .call_method(self.internal, "getService", "()J", &[])?
            .j()?;

        Ok(ArcPointer::of(service_l).arc())
    }

    pub (crate) fn set_service(&self, service: Arc<dyn Service<Protocol = Test>>) -> Result<()> {
        self.env
            .call_method(
                self.internal,
                "setService",
                "(J)V",
                &[JValue::Long(ArcPointer::new(service).into())],
            )?
            .v()
    }

    pub (crate) fn get_executor(&self) -> Result<&mut ThreadPool> {
        let tpl = self
            .env
            .call_method(self.internal, "getExecutor", "()J", &[])?
            .j()?;

        let tpr = tpl as *mut ThreadPool;
        let tp = Box::leak(unsafe { Box::from_raw(tpr) });

        Ok(tp)
    }

    pub (crate) fn set_executor(&self, tp: ThreadPool) -> Result<()> {
        let tpl = Box::into_raw(Box::new(tp)) as *const () as i64;

        self.env
            .call_method(self.internal, "setExecutor", "(J)V", &[JValue::Long(tpl)])?
            .v()
    }
}