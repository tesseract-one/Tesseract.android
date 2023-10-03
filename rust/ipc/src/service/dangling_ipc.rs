//!!!!!THIS IS JUST AND EXAMPLE (which was a former impl of IPCTransport)
//! use code from here to implement rust transports importing to java

use jni::objects::JObject;
use jni::JNIEnv;
use jni::sys::jlong;

use jni_fn::jni_fn;

use interop_android::error::deresultify;

use tesseract_ipc_android::service::Transport;

use tesseract_ipc_android::service::Applicator;


#[jni_fn("one.tesseract.service.transport.IPCTransport")]
pub fn createApplicator<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> jlong {
    deresultify(&env, || {
        //debug!("!!!INSIDE CREATE APPLICATOR");
        let chennel = env.call_method(this, "getChannel", "()Ljava/lang/String;", &[])?.l()?;
        //debug!("!!!GOT CHANNEL");
        let channel: String = env.get_string(chennel.into())?.into();

        let transport = Transport::new(&env, &channel)?;

        let applicator: Box<dyn Applicator> = Box::new(move |tesseract| {
            tesseract.transport(transport)
        });

        Ok(Box::into_raw(Box::new(applicator)) as jlong)
    })
}