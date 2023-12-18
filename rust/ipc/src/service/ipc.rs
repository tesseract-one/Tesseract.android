use jni::{JNIEnv, objects::JObject, errors::Result};

use tesseract_one::service::Transport;
use tesseract_android_transport::service::JTransport;

const TRANSPORT_CLASS: &str = "one/tesseract/service/transport/ipc/IPCTransport";

pub struct IPCTransport {
    internal: JTransport
}

impl IPCTransport {
    pub fn from_transport<'a: 'b, 'b>(env: &'b JNIEnv<'a>, transport: JObject<'a>) -> Result<Self> {
        trace!("About to create IPCTransport from java object");
        let transport = JTransport::from_local(env, transport)?;
        Ok(Self {
            internal: transport
        })
    }

    pub fn new(env: &JNIEnv, channel: &str) -> Result<Self> {
        trace!("About to create IPCTransport with channel: {}", channel);
        let channel = env.new_string(channel)?;
        let transport = env.new_object(TRANSPORT_CLASS, "(Ljava/lang/String;)V", &[channel.into()])?;
        Self::from_transport(env, transport)
    }

    pub fn default(env: &JNIEnv) -> Result<Self> {
        trace!("About to create a default IPCTransport");
        let transport = env.call_static_method(TRANSPORT_CLASS, "default", "()Lone/tesseract/service/transport/ipc/IPCTransport;", &[])?.l()?;
        Self::from_transport(env, transport)
    }
}

impl Transport for IPCTransport {
    fn bind(self, processor: std::sync::Arc<dyn tesseract_one::service::TransportProcessor + Send + Sync>) -> Box<dyn tesseract_one::service::BoundTransport + Send> {
        trace!("About to bind the IPCTransport");
        self.internal.bind(processor)
    }
}