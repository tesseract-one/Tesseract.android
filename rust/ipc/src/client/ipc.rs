use std::sync::Arc;

use async_trait::async_trait;

use jni::{JNIEnv, objects::JObject, errors::Result};

use tesseract_one::{client::{Transport, transport::Status, Connection}, Protocol};
use tesseract_android_transport::client::RJTransport;

const TRANSPORT_CLASS: &str = "one/tesseract/client/transport/ipc/IPCTransport";

pub struct IPCTransport {
    internal: Arc<RJTransport>
}

impl IPCTransport {
    pub fn from_transport<'a: 'b, 'b>(env: &'b JNIEnv<'a>, transport: JObject<'a>) -> Result<Self> {
        trace!("About to create IPCTransport from java object");
        let transport = RJTransport::from_jobject(env, transport)?;
        Ok(Self {
            internal: Arc::new(transport)
        })
    }

    pub fn new<'a: 'b, 'b>(env: &'b JNIEnv<'a>, applcation: JObject<'a>) -> Result<Self> {
        trace!("About to create IPCTransport");
        let transport = env.new_object(TRANSPORT_CLASS, "(Landroid/app/Application;)V", &[applcation.into()])?;
        Self::from_transport(env, transport)
    }
}

#[async_trait]
impl Transport for IPCTransport {
    fn id(&self) -> String {
        self.internal.id()
    }

    async fn status(self: Arc<Self>, protocol: Box<dyn Protocol>) -> Status {
        let internal = Arc::clone(&self.internal);
        internal.status(protocol).await
    }

    fn connect(&self, protocol: Box<dyn Protocol>) -> Box<dyn Connection + Sync + Send> {
        self.internal.connect(protocol)
    }
}