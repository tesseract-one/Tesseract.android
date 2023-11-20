use jni::objects::JObject;
use jni::JNIEnv;
use jni::errors::Error;

use jni_fn::jni_fn;

use crabdroid::error::JavaErrorContext;

use tesseract::client::{Tesseract, Delegate};
use tesseract_android_base::TesseractAndroidError;
use tesseract_android_base::service::Applicator;

const PTR_FIELD: &str = "ptr";

type Del = tesseract::client::delegate::SingleTransportDelegate;

#[jni_fn("one.tesseract.client.Tesseract")]
pub fn create<'a>(env: JNIEnv<'a>, this: JObject<'a>, delegate: JObject<'a>) {
    TesseractAndroidError::java_context(&env, || {
        let delegate = Del::arc(); //TODO: wrap actual delegate

        let tesseract = Tesseract::new(delegate);
        unsafe {env.set_rust_field(this, PTR_FIELD, tesseract)?};
        Ok(())
    })
}

#[jni_fn("one.tesseract.service.Tesseract")]
pub fn finalize(env: JNIEnv, this: JObject) {
    Error::java_context(&env, || {
        let tesseract: Tesseract<Del> = unsafe {env.take_rust_field(this, PTR_FIELD)}?;
        Ok(drop(tesseract))
    })
}