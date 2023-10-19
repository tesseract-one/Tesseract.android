use jni::{JNIEnv, objects::JObject, errors::Result};

use tesseract::service::Transport;
use tesseract_android_transport::service::JTransport;

const TRANSPORT_CLASS: &str = "one/tesseract/service/transport/ipc/IPCTransport";

pub struct IPCTransport {
    internal: JTransport
}

impl IPCTransport {
    pub fn from_transport<'a: 'b, 'b>(env: &'b JNIEnv<'a>, transport: JObject<'a>) -> Result<Self> {
        let transport = JTransport::from_local(env, transport)?;
        Ok(Self {
            internal: transport
        })
    }

    pub fn new(env: &JNIEnv, channel: &str) -> Result<Self> {
        let channel = env.new_string(channel)?;
        let transport = env.new_object(TRANSPORT_CLASS, "(Ljava/lang/String;)V", &[channel.into()])?;
        Self::from_transport(env, transport)
    }

    pub fn default(env: &JNIEnv) -> Result<Self> {
        let transport = env.call_static_method(TRANSPORT_CLASS, "default", "()Lone/tesseract/service/transport/ipc/IPCTransport;", &[])?.l()?;
        Self::from_transport(env, transport)
    }
}

impl Transport for IPCTransport {
    fn bind(self, processor: std::sync::Arc<dyn tesseract::service::TransportProcessor + Send + Sync>) -> Box<dyn tesseract::service::BoundTransport + Send> {
        self.internal.bind(processor)
    }
}