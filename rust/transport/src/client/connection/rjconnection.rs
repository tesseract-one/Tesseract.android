use std::sync::Arc;
use async_trait::async_trait;

use errorcon::convertible::ErrorContext;

use jni::{JNIEnv, objects::JObject};
use crabdroid::ContextedGlobal;

use tesseract_one::client::Connection;
use tesseract_android_base::TesseractAndroidError;

use jni::errors::Result as JResult;
use tesseract_one::Result as TResult;

use super::jconnection::JConnection;

pub (crate) struct RJConnection {
    jconnection: ContextedGlobal
}

impl RJConnection {
    pub fn from_jobject<'a: 'b, 'b>(env: &'b JNIEnv<'a>, jdelegate: JObject<'a>) -> JResult<Self> {
        debug!("Creating connection");
        let global = ContextedGlobal::from_local(env, jdelegate)?;
        Ok(Self {
            jconnection: global
        })
    }
}

#[async_trait]
impl Connection for RJConnection {
    async fn send(self: Arc<Self>, request: Vec<u8>) -> TResult<()> {
        TesseractAndroidError::context_async(async || {
            //we can ignore the result as it's void. If there is an error, we get it through ?
            let _ = self.jconnection.with_async_context(32, |env, jconnection| {
                let jconnection = JConnection::from_env(env, jconnection);
                jconnection.send(request)
            }).await?;
            Ok(())
        }).await
    }

    async fn receive(self: Arc<Self>) -> TResult<Vec<u8>> {
        TesseractAndroidError::context_async(async || {
            let data = self.jconnection.with_async_context(32, |env, jconnection| {
                let jconnection = JConnection::from_env(env, jconnection);
                jconnection.receive()
            }).await?;

            let data = data.with_safe_context_rret(32, |env, data| {
                env.convert_byte_array(data.into_raw())
            })?;

            Ok(data)
        }).await
    }
}