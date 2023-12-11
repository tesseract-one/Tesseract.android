use jni::{JNIEnv, objects::JObject, errors::{Result, Error}};
use crabdroid::future::JCompletionStage;

use super::RJConnection;

/// Lifetime'd representation of a `Delegate`. Just a `JObject` wrapped in a
/// new class.
#[derive(Clone, Copy)]
pub struct JConnection<'a: 'b, 'b> {
    object: JObject<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> ::std::ops::Deref for JConnection<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<'a: 'b, 'b> From<JConnection<'a, 'b>> for JObject<'a> {
    fn from(other: JConnection<'a, 'b>) -> JObject<'a> {
        other.object
    }
}

impl<'a: 'b, 'b> TryInto<RJConnection> for JConnection<'a, 'b> {
    type Error = Error;

    fn try_into(self) -> std::prelude::v1::Result<RJConnection, Self::Error> {
        RJConnection::from_jobject(self.env, self.object)
    }
}

impl<'a: 'b, 'b> JConnection<'a, 'b> {
    pub fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> JConnection<'a, 'b> {
        JConnection {
            object: obj,
            env: env,
        }
    }

    pub fn send(&self, data: Vec<u8>) -> Result<JCompletionStage<'a, 'b>> { //Void
        let bytes = self.env.byte_array_from_slice(&data)?;
        let bytes = unsafe {JObject::from_raw(bytes)};

        let stage = self.env.call_method(
            self.object,
            "send",
            "([B)Ljava/util/concurrent/CompletionStage;",
            &[bytes.into()])?.l()?;

        let stage = JCompletionStage::from_env(self.env, stage);

        Ok(stage)
    }

    pub fn receive(&self) -> Result<JCompletionStage<'a, 'b>> { //ByteArray
        let stage = self.env.call_method(
            self.object,
            "receive",
            "()Ljava/util/concurrent/CompletionStage;",
            &[])?.l()?;

        let stage = JCompletionStage::from_env(self.env, stage);

        Ok(stage)
    }
}