use jni::{JNIEnv,
    objects::JObject,
    errors::Result
};

pub (crate) trait IntoJava {
    fn into_java<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<JObject<'a>>;
}