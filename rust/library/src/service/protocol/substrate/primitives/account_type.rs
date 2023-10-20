use crabdroid::env::AndroidEnv;

use jni::{JNIEnv,
    objects::JObject,
    errors::Result
};

use crate::utils::IntoJava;

pub (crate) const ACCOUNT_TYPE_CLASS_NAME: &str = "one/tesseract/service/protocol/common/substrate/AccountType";

impl IntoJava for tesseract_protocol_substrate::AccountType {
    fn into_java<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<JObject<'a>> {
        let clazz = env.find_class_android(ACCOUNT_TYPE_CLASS_NAME)?;
        let sig = format!("L{};", ACCOUNT_TYPE_CLASS_NAME);

        Ok(match self {
            tesseract_protocol_substrate::AccountType::Ed25519 => {
                env.get_static_field(clazz, "Ed25519", &sig)?.l()?
            },
            tesseract_protocol_substrate::AccountType::Sr25519 => {
                env.get_static_field(clazz, "Sr25519", &sig)?.l()?
            },
            tesseract_protocol_substrate::AccountType::Ecdsa => {
                env.get_static_field(clazz, "Ecdsa", &sig)?.l()?
            },
        })
    }
}

