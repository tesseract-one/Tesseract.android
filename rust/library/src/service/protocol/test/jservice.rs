use crabdroid::future::completion_stage::JCompletionStage;

use jni::{JNIEnv, objects::{JObject, JString, JValue}, errors::Result};

/// Lifetime'd representation of a `TestService`. Just a `JObject` wrapped in a
/// new class.
#[derive(Clone, Copy)]
pub (super) struct JTestService<'a: 'b, 'b> {
    internal: JObject<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> ::std::ops::Deref for JTestService<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a: 'b, 'b> From<JTestService<'a, 'b>> for JObject<'a> {
    fn from(other: JTestService<'a, 'b>) -> JObject<'a> {
        other.internal
    }
}

impl<'a: 'b, 'b> JTestService<'a, 'b> {
    pub (super) fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> JTestService<'a, 'b> {
       JTestService {
            internal: obj,
            env: env,
        }
    }

    pub (super) fn is_test_service(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> Result<bool> {
        Ok(
            !(env.is_same_object(obj, JObject::null())?) &&
                env.is_instance_of(obj, "one/tesseract/service/protocol/java/TestService")?
        )
    }

    pub (super) fn sign_transaction(&self, request: JString<'a>) -> Result<JCompletionStage<'a, 'b>> {
        let stage = self.env
            .call_method(
                self.internal,
                "signTransaction",
                "(Ljava/lang/String;)Ljava/util/concurrent/CompletionStage;",
                &[JValue::from(request)]
            )?
            .l()?;

        Ok(JCompletionStage::from_env(self.env, stage))
    }
}