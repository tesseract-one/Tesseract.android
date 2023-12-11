use jni_fn::jni_fn;

use jni::objects::JObject;
use jni::JNIEnv;
use jni::sys::jlong;

use crabdroid::JavaErrorContext;

use tesseract_android_base::TesseractAndroidError;
use tesseract_android_base::client::Applicator;

use super::IPCTransport;


#[jni_fn("one.tesseract.client.transport.ipc.IPCTransportR")]
pub fn createApplicator<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> jlong {
    TesseractAndroidError::java_context(&env, || {
        let application = env.call_method(this, "getApplication", "()Landroid/app/Application;", &[])?.l()?;
        
        let transport = IPCTransport::new(&env, application);

        let applicator: Box<dyn Applicator> = Box::new(move |tesseract| {
            tesseract.transport(transport)
        });

        Ok(Box::into_raw(Box::new(applicator)) as jlong)
    })
}