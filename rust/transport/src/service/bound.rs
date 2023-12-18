use jni::objects::JObject;
use jni::JNIEnv;
use jni::errors::Result;

use crabdroid::ContextedGlobal;

use tesseract_one::service::BoundTransport;

pub (super) struct JBoundTransport {
    _bound: ContextedGlobal
}

impl JBoundTransport {
    pub fn from_local(env: &JNIEnv, local: JObject) -> Result<Self> {
        Ok(Self {
            _bound: ContextedGlobal::from_local(env, local)?
        })
    }
}

impl BoundTransport for JBoundTransport {
}