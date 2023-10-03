use std::sync::Arc;

use interop_android::error::JavaErrorContext;
use interop_android::ContextedGlobal;

use jni::objects::{JObject, JValue};
use jni::JNIEnv;
use jni::errors::{Error, Result};
use jni::sys::jlong;

use jni_fn::jni_fn;

use tesseract::service::{TransportProcessor, Transport, BoundTransport};

use tesseract_android_base::service::Applicator;

use super::bound::JBoundTransport;
use super::processor::JProcessor;

pub struct JTransport {
    internal: ContextedGlobal
}

impl JTransport {
    pub fn from_local(env: &JNIEnv, local: JObject) -> Result<Self> {
        Ok(Self {
            internal: ContextedGlobal::from_local(env, local)?
        })
    }
}

impl Transport for JTransport {
    fn bind(self, processor: Arc<dyn TransportProcessor + Send + Sync>) -> Box<dyn BoundTransport + Send> {
        Box::new(self.internal.with_safe_context_rret(64, |env, transport| {
            let processor = JProcessor::new(&env, processor)?;

            debug!("!!!!PROCESSOR CREATED!!!");

            let processor: JValue = JValue::Object(processor.into());

            debug!("!!!!PROCESSOR CREATED VALUE!!!");

            let bound = env.call_method(
                transport,
                "bind",
                "(Lone/tesseract/transport/service/Processor;)Lone/tesseract/transport/service/BoundTransport;",
                &[processor])?.l()?;

            debug!("!!!!AND BOUND!!!");

            JBoundTransport::from_local(env, bound)
        })
        .inspect_err(|e| {
            match e {
                interop_android::error::GlobalError::Exception(exception) => {
                    debug!("!!!!AND HERE IS THE #$%!!!: {}", e);
                    exception.do_in_context_rret(64, |env, exception| {
                        env.call_method(exception, "printStackTrace", "()V", &[])?.v()
                    }).unwrap()
                },
                interop_android::error::GlobalError::JniError(e) => {
                    debug!("!!!!AND HERE IS THE JNI ERROR!!!: {}", e);
                },
            }
        })
        .expect("Maybe improve the rust transport API to be able to provide an error here?"))
    }
}

#[jni_fn("one.tesseract.transport.service.JavaRustTransport")]
pub fn createApplicator<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> jlong {
    Error::java_context(&env, || {
        let transport = env.get_field(this, "transport", "Lone/tesseract/transport/service/JavaTransport;")?.l()?;
        
        let transport = JTransport::from_local(&env, transport)?;

        let applicator: Box<dyn Applicator> = Box::new(move |tesseract| {
            tesseract.transport(transport)
        });

        Ok(Box::into_raw(Box::new(applicator)) as jlong)
    })
}