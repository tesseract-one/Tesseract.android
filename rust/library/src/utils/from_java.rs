use jni::{
    JNIEnv,
    objects::JObject,
    errors::Result
};

pub (crate) trait FromJava: Sized {
    fn from_java<'a: 'b, 'b>(env: &'b JNIEnv<'a>, jobject: JObject<'a>) -> Result<Self>;
}