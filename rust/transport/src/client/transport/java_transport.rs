use jni::{JNIEnv, objects::JObject};
use jni::errors::Error;
use jni::sys::jlong;
use jni_fn::jni_fn;

use crabdroid::JavaErrorContext;

use tesseract_android_base::client::Applicator;

use super::rjtransport::RJTransport;

#[jni_fn("one.tesseract.client.transport.JavaRustTransport")]
pub fn createApplicator<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> jlong {
    Error::java_context(&env, || {
        let transport = env.get_field(
            this,
            "transport",
            "Lone/tesseract/client/transport/JavaTransport;")?.l()?;
        
        let transport = RJTransport::from_jobject(&env, transport)?;

        let applicator: Box<dyn Applicator> = Box::new(move |tesseract| {
            tesseract.transport(transport)
        });

        Ok(Box::into_raw(Box::new(applicator)) as jlong)
    })
}