use jni::JNIEnv;
use jni::objects::JObject;
use jni::errors::Result;

use crabdroid::env::AndroidEnv;

use tesseract_protocol_substrate::AccountType;

use crate::utils::FromJava;

pub (crate) const ACCOUNT_TYPE_CLASS_NAME: &str = "one/tesseract/protocol/common/substrate/AccountType";

impl FromJava for AccountType {
    fn from_java<'a: 'b, 'b>(env: &'b JNIEnv<'a>, jobject: JObject<'a>) -> Result<Self> {
        let clazz = env.find_class_android(ACCOUNT_TYPE_CLASS_NAME)?;
        let sig = format!("L{};", ACCOUNT_TYPE_CLASS_NAME);

        Ok(if env.is_same_object(jobject, env.get_static_field(clazz, "Ed25519", &sig)?.l()?)? {
            Self::Ed25519
        } else if env.is_same_object(jobject, env.get_static_field(clazz, "Sr25519", &sig)?.l()?)? {
            Self::Sr25519
        } else if env.is_same_object(jobject, env.get_static_field(clazz, "Ecdsa", &sig)?.l()?)? {
            Self::Ecdsa
        } else {
            panic!("Invalid account type. Can't convert to Rust")
        })
    }
}