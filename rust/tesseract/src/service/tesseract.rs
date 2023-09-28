use jni::objects::{JObject, JValue};
use jni::JNIEnv;
use jni::errors::{Error, Result};

use jni_fn::jni_fn;

use interop_android::env::AndroidEnv;
use interop_android::error::Deresultify;
use tesseract::service::Tesseract;

use super::{service, transport};

use tesseract_ipc_android::service::Applicator;

const PTR_FIELD: &str = "ptr";

#[jni_fn("one.tesseract.service.Tesseract")]
pub fn create<'a>(env: JNIEnv<'a>, this: JObject<'a>) {
    Error::deresultify(&env, || {
        android_log::init("TESSERACTNNNN").unwrap(); //TODO: custom error
        let tesseract = Tesseract::new();
        unsafe {env.set_rust_field(this, PTR_FIELD, tesseract)?};
        Ok(())
    })
}

#[jni_fn("one.tesseract.service.Tesseract")]
pub fn service<'a>(env: JNIEnv<'a>, this: JObject<'a>, service: JObject<'a>) -> JObject<'a> {
    Error::deresultify(&env, || {
        let mut tesseract: Tesseract = unsafe {env.take_rust_field(this, PTR_FIELD)}?;

        let applicators  = service::jservice_to_services(&env, service)?;

        for applicator in applicators {
            tesseract = applicator(tesseract)
        }

        unsafe {env.set_rust_field(this, PTR_FIELD, tesseract)?};

        Ok(this)
    })
}

#[jni_fn("one.tesseract.service.Tesseract")]
pub fn transport<'a>(env: JNIEnv<'a>, this: JObject<'a>, transport: JObject<'a>) -> JObject<'a> {
    Error::deresultify(&env, || {
        let transport = env.call_method(
            transport,
            "rustTransport",
            "()Lone/tesseract/transport/service/RustTransport;",
            &[])?.l()?;

        let transport = env.call_method(
            transport,
            "createApplicator",
            "()J",
            &[])?.j()?;

        let applicator = *unsafe {
            Box::from_raw(transport as *mut Box<dyn Applicator>)
        };

        let tesseract: Tesseract = unsafe {env.take_rust_field(this, PTR_FIELD)}?;

        let tesseract = applicator(tesseract);

        unsafe {env.set_rust_field(this, PTR_FIELD, tesseract)?};

        Ok(this)
    })
}

#[jni_fn("one.tesseract.service.Tesseract")]
pub fn finalize(env: JNIEnv, this: JObject) {
    Error::deresultify(&env, || {
        let tesseract: Tesseract = unsafe {env.take_rust_field(this, PTR_FIELD)}?;
        Ok(drop(tesseract))
    })
}