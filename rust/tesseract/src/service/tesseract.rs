use jni::objects::{JObject, JValue};
use jni::JNIEnv;
use jni::errors::Result;

use jni_fn::jni_fn;

use interop_android::env::AndroidEnv;
use interop_android::pointer::ArcPointer;
use interop_android::error::deresultify;
use tesseract::service::Tesseract;

use super::service;

const PTR_FIELD: &str = "ptr";

#[jni_fn("one.tesseract.service.Tesseract")]
pub fn create<'a>(env: JNIEnv<'a>, this: JObject<'a>) {
    deresultify(&env, || {
        let tesseract = Tesseract::new();
        unsafe {env.set_rust_field(this, PTR_FIELD, tesseract)?};
        Ok(())
    })
}

#[jni_fn("one.tesseract.service.Tesseract")]
pub fn addService<'a>(env: JNIEnv<'a>, this: JObject<'a>, service: JObject<'a>) {
    deresultify(&env, || {
        let mut tesseract: Tesseract = unsafe {env.take_rust_field(this, "handle")}?;

        let applicators  = service::jservice_to_services(&env, service)?;

        for applicator in applicators {
            tesseract = applicator(tesseract)
        }

        unsafe {env.set_rust_field(this, PTR_FIELD, tesseract)?};
        Ok(())
    })
}

#[jni_fn("one.tesseract.service.Tesseract")]
pub fn addTransport<'a>(env: JNIEnv<'a>, this: JObject<'a>, transport: JObject<'a>) {
    deresultify(&env, || {
        let tesseract: Tesseract = unsafe {env.take_rust_field(this, "handle")}?;
        let tesseract = Tesseract::new();//tes.transport
        unsafe {env.set_rust_field(this, PTR_FIELD, tesseract)?};
        Ok(())
    })
}

#[jni_fn("one.tesseract.service.Tesseract")]
pub fn finalize(env: JNIEnv, this: JObject) {
    deresultify(&env, || {
        let tesseract: Tesseract = unsafe {env.take_rust_field(this, PTR_FIELD)}?;
        Ok(drop(tesseract))
    })
}