use std::collections::HashMap;
use async_trait::async_trait;

use crabdroid::ContextedGlobal;
use jni::{JNIEnv, objects::JObject, errors::Result};
use tesseract_one::client::{transport::Status, Delegate};

use super::jdelegate::JDelegate;

pub (crate) struct RJDelegate {
    jdelegate: ContextedGlobal
}

impl RJDelegate {
    pub fn from_jobject<'a: 'b, 'b>(env: &'b JNIEnv<'a>, jdelegate: JObject<'a>) -> Result<Self> {
        let global = ContextedGlobal::from_local(env, jdelegate)?;
        Ok(Self {
            jdelegate: global
        })
    }
}

#[async_trait]
impl Delegate for RJDelegate {
    async fn select_transport(
        &self,
        transports: &HashMap<String, Status>,
    ) -> Option<String> {
        let transports = HashMap::clone(&transports);
        let gid = self.jdelegate.with_async_context(32, |env, jdelegate| {
            let jdelegate = JDelegate::from_env(env, jdelegate);
            Ok(jdelegate.select_transport(transports)?)
        }).await.expect("System error: Java interop looks to be broken (RJDelegate)");

        gid.with_safe_context_rret(32, |env, jid| {
            if env.is_same_object(jid, JObject::null())? {
                Ok(None)
            } else {
                let id: String = env.get_string(jid.into())?.into();
                Ok(Some(id))
            }
        }).expect("System error: Java interop looks to be broken (RJDelegate2)")
    }
}