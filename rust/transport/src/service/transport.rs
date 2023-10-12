use std::sync::Arc;

use errorcon::simple::ErrorContext;

use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use jni::errors::{Error, Result};
use jni::sys::jlong;

use jni_fn::jni_fn;

use crabdroid::{JavaErrorContext, ContextedGlobal};

use tesseract::service::{TransportProcessor, Transport, BoundTransport};

use tesseract_android_base::TesseractAndroidError;
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
        Box::new(TesseractAndroidError::context_panicable(|| {
            Ok(self.internal.with_safe_context_rret(64, |env, transport| {
                debug!("About to create JProcessor");
                let processor = JProcessor::new(&env, processor)?;
                debug!("JProcessor created successfully");
                let processor: JValue = JValue::Object(processor.into());
                debug!("JProcessor value created");
    
                let bound = env.call_method(
                    transport,
                    "bind",
                    "(Lone/tesseract/transport/service/Processor;)Lone/tesseract/transport/service/BoundTransport;",
                    &[processor])?.l()?;
    
                debug!("JProcessor bound successfully");
    
                JBoundTransport::from_local(env, bound)
            })?)
        }))
    }
}

#[jni_fn("one.tesseract.transport.service.JavaRustTransport")]
pub fn createApplicator<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> jlong {
    Error::java_context(&env, || {
        let transport = env.get_field(
            this,
            "transport",
            "Lone/tesseract/transport/service/JavaTransport;")?.l()?;
        
        let transport = JTransport::from_local(&env, transport)?;

        let applicator: Box<dyn Applicator> = Box::new(move |tesseract| {
            tesseract.transport(transport)
        });

        Ok(Box::into_raw(Box::new(applicator)) as jlong)
    })
}