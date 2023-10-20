use jni::{
    JNIEnv,
    objects::{JObject, JValue},
    errors::Result
};

use crabdroid::JavaConvertibleDesc;

use tesseract_protocol_substrate::GetAccountResponse;

use crate::utils::Wrapper;

impl JavaConvertibleDesc for Wrapper<GetAccountResponse> {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/service/protocol/common/substrate/GetAccountResponse"
    }

    fn fields() -> Vec<(&'static str, &'static str)> {
        [
            ("publicKey", "[B"),
            ("path", "Ljava/lang/String;"),
        ].into()
    }

    fn into_values<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<Vec<jni::objects::JValue<'a>>> {
        let key = env.byte_array_from_slice(&self.inner.public_key)?;
        let path = env.new_string(&self.inner.path)?;

        let key = JValue::from(unsafe { JObject::from_raw(key) });
        let path = JValue::from(path);

        Ok(vec![key, path])
    }

    fn from_values<'a: 'b, 'b>(env: &'b JNIEnv<'a>, values: &[jni::objects::JValue<'a>]) -> Result<Self> {
        let key = values[0].l()?;
        let path = values[1].l()?;

        let key = env.convert_byte_array(key.into_raw())?;
        let path: String = env.get_string(path.into())?.into();

        Ok(GetAccountResponse {
            public_key: key,
            path: path,
        }.into())
    }
}