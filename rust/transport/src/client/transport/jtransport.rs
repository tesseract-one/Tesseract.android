use jni::{JNIEnv, objects::JObject, errors::Result};
use crabdroid::future::JCompletionStage;

use super::super::connection::JConnection;

/// Lifetime'd representation of a `Delegate`. Just a `JObject` wrapped in a
/// new class.
#[derive(Clone, Copy)]
pub struct JTransport<'a: 'b, 'b> {
    object: JObject<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> ::std::ops::Deref for JTransport<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<'a: 'b, 'b> From<JTransport<'a, 'b>> for JObject<'a> {
    fn from(other: JTransport<'a, 'b>) -> JObject<'a> {
        other.object
    }
}

impl<'a: 'b, 'b> JTransport<'a, 'b> {
    pub fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> JTransport<'a, 'b> {
        JTransport {
            object: obj,
            env: env,
        }
    }

    pub fn id(&self) -> Result<String> {
        let id = self.env.call_method(self.object, "id", "()Ljava/lang/String;", &[])?.l()?;
        let id: String = self.env.get_string(id.into())?.into();
        Ok(id)
    }

    pub fn status(&self, protocol: &str) -> Result<JCompletionStage<'a, 'b>> { //Status
        let protocol = self.env.new_string(protocol)?;
        let stage = self.env.call_method(
            self.object,
            "status",
            "(Ljava/lang/String;)Ljava/util/concurrent/CompletionStage;",
            &[protocol.into()])?.l()?;
        let stage = JCompletionStage::from_env(self.env, stage);
        Ok(stage)
    }

    pub fn connect(&self, protocol: &str) -> Result<JConnection<'a, 'b>> {
        let protocol = self.env.new_string(protocol)?;
        let connection = self.env.call_method(
            self.object,
            "connect",
            "(Ljava/lang/String;)Lone/tesseract/client/transport/Connection;",
            &[protocol.into()])?.l()?;
        let connection = JConnection::from_env(self.env, connection);
        Ok(connection)
    }
}