use std::collections::HashMap;

use jni::{JNIEnv, objects::{JObject, JValue, JMap}, errors::Result};
use crabdroid::future::JCompletionStage;
use tesseract::client::transport::Status;
use tesseract_android_transport::client::JStatusConvertible;

/// Lifetime'd representation of a `Delegate`. Just a `JObject` wrapped in a
/// new class.
#[derive(Clone, Copy)]
pub struct JDelegate<'a: 'b, 'b> {
    object: JObject<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> ::std::ops::Deref for JDelegate<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<'a: 'b, 'b> From<JDelegate<'a, 'b>> for JObject<'a> {
    fn from(other: JDelegate<'a, 'b>) -> JObject<'a> {
        other.object
    }
}

impl<'a: 'b, 'b> JDelegate<'a, 'b> {
    pub fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> JDelegate<'a, 'b> {
        JDelegate {
            object: obj,
            env: env,
        }
    }

    pub fn select_transport(&self, transports: HashMap<String, Status>) -> Result<JCompletionStage<'a, 'b>> { //Future of nullable String
        let map_size = JValue::Int(transports.len().try_into().expect("yeah... sure"));
        let jmap = self.env.new_object("java/util/HashMap", "(I)V", &[map_size])?;
        let jmap = JMap::from_env(self.env, jmap)?;

        for (tid, status) in transports {
            let _ = self.env.with_local_frame(16, || {
                let jid = self.env.new_string(&tid)?;
                let jstatus = status.into_java(self.env)?;
                let _ = jmap.put(jid.into(), jstatus)?;
                Ok(JObject::null())
            })?;
        }

        let cstage = self
            .env
            .call_method(self.object, "selectTransport", "(Ljava/util/Map;)Ljava/util/concurrent/CompletionStage;", &[jmap.into()])?
            .l()?;

        Ok(JCompletionStage::from_env(self.env, cstage))
    }
}
