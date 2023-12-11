use std::sync::{MutexGuard, Arc};

use jni::objects::{JObject, JString};
use jni::JNIEnv;
use jni::errors::Error;

use jni_fn::jni_fn;

use crabdroid::error::JavaErrorContext;

use tesseract::client::Delegate;
use tesseract::client::{
    Tesseract,
    delegate::SingleTransportDelegate,
};
use tesseract_android_base::TesseractAndroidError;
use tesseract_android_base::client::Applicator;

use super::delegate::RJDelegate;
use super::service;

const PTR_FIELD: &str = "ptr";

#[jni_fn("one.tesseract.client.Tesseract")]
pub fn create<'a>(env: JNIEnv<'a>, this: JObject<'a>, delegate: JObject<'a>) {
    TesseractAndroidError::java_context(&env, || {
        let delegate: Arc<dyn Delegate + Send + Sync + 'static> = if env.is_same_object(delegate, JObject::null())? {
            SingleTransportDelegate::arc()
        } else {
            let delegate = RJDelegate::from_jobject(&env, delegate)?;
            Arc::new(delegate)
        };

        //let application = env.get_field(this, "application", "Landroid.app.Application;")?.l()?;

        //let application = env.call_method(this, "getApplication", "()Landroid/app/Application;", &[])?.l()?;

        //let ipc = tesseract_android_ipc::client::IPCTransport::new(&env, application);

        let tesseract = Tesseract::new(delegate);//.transport(ipc);
        unsafe {env.set_rust_field(this, PTR_FIELD, tesseract)?};
        Ok(())
    })
}

#[jni_fn("one.tesseract.client.Tesseract")]
pub fn transport<'a>(env: JNIEnv<'a>, this: JObject<'a>, transport: JObject<'a>) -> JObject<'a> {
    Error::java_context(&env, || {
        let transport = env.call_method(
            transport,
            "rustTransport",
            "()Lone/tesseract/common/transport/RustTransport;",
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

#[jni_fn("one.tesseract.client.Tesseract")]
pub fn service<'a>(env: JNIEnv<'a>, this: JObject<'a>, name: JString<'a>) -> JObject<'a> {
    TesseractAndroidError::java_context(&env, || {
        let name: String = env.get_string(name)?.into();

        let tesseract: MutexGuard<Tesseract> = unsafe {env.get_rust_field(this, PTR_FIELD)}?;

        Ok(service::java_service_by_name(&env, &tesseract, &name)?)
    })
}

#[jni_fn("one.tesseract.client.Tesseract")]
pub fn finalize(env: JNIEnv, this: JObject) {
    Error::java_context(&env, || {
        let tesseract: Tesseract = unsafe {env.take_rust_field(this, PTR_FIELD)}?;
        Ok(drop(tesseract))
    })
}