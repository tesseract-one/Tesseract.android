use std::sync::Arc;
use async_trait::async_trait;

use jni::{JNIEnv, objects::JObject, errors::Result};
use crabdroid::ContextedGlobal;

use tesseract::{client::{transport::Status, Transport, Connection}, Protocol};
use tesseract_android_base::TesseractAndroidError;

use super::super::{connection::RJConnection, status::JStatusConvertible};
use super::jtransport::JTransport;

pub (crate) struct RJTransport {
    jtransport: ContextedGlobal
}

impl RJTransport {
    pub fn from_jobject<'a: 'b, 'b>(env: &'b JNIEnv<'a>, jdelegate: JObject<'a>) -> Result<Self> {
        let global = ContextedGlobal::from_local(env, jdelegate)?;
        Ok(Self {
            jtransport: global
        })
    }
}

#[async_trait]
impl Transport for RJTransport {
    fn id(&self) -> String {
        self.jtransport.with_safe_context_rret(32, |env, jtransport| {
            let jtransport = JTransport::from_env(env, jtransport);
            jtransport.id()
        }).expect("Runtime java error in RJTransport::id")
    }

    async fn status(self: Arc<Self>, protocol: Box<dyn Protocol>) -> Status {
        let protocol = protocol.id();
        let status = self.jtransport.with_async_context(32, |env, jtransport| {
            let jtransport = JTransport::from_env(env, jtransport);
            jtransport.status(&protocol)
        }).await;

        let status = status.and_then(|status| {
            status.with_safe_context_rret(32, |env, jstatus| {
                Status::from_java(env, jstatus)
            })
        });

        match status {
            Ok(status) => status,
            Err(e) => Status::Error(TesseractAndroidError::Gllobal(e).into()),
        }
    }

    fn connect(&self, protocol: Box<dyn Protocol>) -> Box<dyn Connection + Sync + Send> {
        let connection: RJConnection = self.jtransport.with_safe_context_rret(32, |env, jtransport| {
            let jtransport = JTransport::from_env(env, jtransport);
            let jconnection = jtransport.connect(&protocol.id())?;
            jconnection.try_into()
        }).expect("Runtime java error in RJTransport::connnect");
        Box::new(connection)
    }
}