use jni::{
    JNIEnv,
    errors::Result,
    objects::{JObject, JString, JValue}
};

use crabdroid::future::JCompletionStage;

use super::primitives::ACCOUNT_TYPE_CLASS_NAME;

/// Lifetime'd representation of a `TestService`. Just a `JObject` wrapped in a
/// new class.
#[derive(Clone, Copy)]
pub (super) struct JSubstrateService<'a: 'b, 'b> {
    internal: JObject<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> ::std::ops::Deref for JSubstrateService<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a: 'b, 'b> From<JSubstrateService<'a, 'b>> for JObject<'a> {
    fn from(other: JSubstrateService<'a, 'b>) -> JObject<'a> {
        other.internal
    }
}

impl<'a: 'b, 'b> JSubstrateService<'a, 'b> {
    pub (super) fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> JSubstrateService<'a, 'b> {
        JSubstrateService {
            internal: obj,
            env: env,
        }
    }

    pub (super) fn is_substrate_service(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> Result<bool> {
        Ok(
            !(env.is_same_object(obj, JObject::null())?) &&
                env.is_instance_of(obj, "one/tesseract/service/protocol/java/SubstrateService")?
        )
    }

    pub (super) fn get_account(&self, account_type: JObject<'a>) -> Result<JCompletionStage<'a, 'b>> { //GetAccountResponse
        let sig = format!("(L{};)Ljava/util/concurrent/CompletionStage;", ACCOUNT_TYPE_CLASS_NAME);

        let stage = self.env
            .call_method(
                self.internal,
                "getAccount",
                &sig,
                &[JValue::from(account_type)]
            )?
            .l()?;

        Ok(JCompletionStage::from_env(self.env, stage))
    }

    pub (super) fn sign_transaction(&self,
        account_type: JObject<'a>,
        account_path: JString<'a>,
        extrinsic_data: JObject<'a>, //bytearray and the next two
        extrinsic_metadata: JObject<'a>,
        extrinsic_types: JObject<'a>) -> Result<JCompletionStage<'a, 'b>> { //returns bytearray
            let sig = format!("(L{};Ljava/lang/String;[B[B[B)Ljava/util/concurrent/CompletionStage;", ACCOUNT_TYPE_CLASS_NAME);

            let args = [
                JValue::from(account_type),
                JValue::from(account_path),
                JValue::from(extrinsic_data),
                JValue::from(extrinsic_metadata),
                JValue::from(extrinsic_types),
            ];

            let stage = self.env
                .call_method(
                    self.internal,
                    "signTransaction",
                    &sig,
                    &args
                )?
                .l()?;

            Ok(JCompletionStage::from_env(self.env, stage))
    }
}