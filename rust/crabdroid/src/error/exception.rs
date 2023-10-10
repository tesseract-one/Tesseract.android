use jni::{JNIEnv, objects::JObject};

pub trait ExceptionConvertible {
    fn to_exception<'a: 'b, 'b>(&self, env: &'b JNIEnv<'a>) -> jni::errors::Result<JObject<'a>>;
}

impl ExceptionConvertible for jni::errors::Error {
    fn to_exception<'a: 'b, 'b>(&self, env: &'b JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        match self {
            jni::errors::Error::JavaException => {
                let exception = env.exception_occurred()?;
                env.exception_clear()?; //let's see how it works. probably we should leave it here
                Ok(exception.into())
                //TODO: wrap into JNIException
            },
            e => {
                let message = e.to_string();
                let message = env.new_string(message)?;
                env.new_object(
                    "one/tesseract/crabdroid/InteropException",
                    "(Ljava/lang.String;)V",
                    &[message.into()])
            }
        }
    }
}