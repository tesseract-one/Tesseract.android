use crabdroid::{env::AndroidEnv, error::{ExceptionConvertible, GlobalError}, Exception, ContextedGlobal};
use jni::{JNIEnv, errors::{Result, Error}, objects::JObject};

use tesseract_one::client::transport::Status;
use tesseract_android_base::TesseractAndroidError;

pub trait JStatusConvertible: Sized {
    fn into_java<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<JObject<'a>>;
    fn from_java<'a: 'b, 'b>(env: &'b JNIEnv<'a>, jstatus: JObject<'a>) -> Result<Self>;
}

const CLASS_READY: &str = "one/tesseract/client/transport/Status$Ready";
const CLASS_UNAVAILABLE: &str = "one/tesseract/client/transport/Status$Unavailable";
const CLASS_ERROR: &str = "one/tesseract/client/transport/Status$Error";

impl JStatusConvertible for Status {
    fn into_java<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<JObject<'a>> {
        match self {
            Status::Ready => {
                let clazz = env.find_class_android(CLASS_READY)?;
                env.new_object(clazz, "()V", &[])
            },
            Status::Unavailable(reason) => {
                let jreason = env.new_string(&reason)?;
                let clazz = env.find_class_android(CLASS_UNAVAILABLE)?;
                env.new_object(clazz, "(Ljava/lang/String;)V", &[jreason.into()])
            },
            Status::Error(error) => {
                let exception = TesseractAndroidError::Tesseract(error).to_exception(env)?;
                let clazz = env.find_class_android(CLASS_ERROR)?;
                env.new_object(clazz, "(Ljava/lang/Exception;)V", &[exception.into()])
            },
        }
    }

    fn from_java<'a: 'b, 'b>(env: &'b JNIEnv<'a>, jstatus: JObject<'a>) -> Result<Self> {
        if env.is_instance_of(jstatus, env.find_class_android(CLASS_READY)?)? {
            Ok(Self::Ready)
        } else if env.is_instance_of(jstatus, env.find_class_android(CLASS_UNAVAILABLE)?)? {
            let jreason = env.call_method(jstatus, "getReason", "(java/lang/String)L;", &[])?.l()?;
            let reason: String = env.get_string(jreason.into())?.into();
            Ok(Self::Unavailable(reason))
        } else if env.is_instance_of(jstatus, env.find_class_android(CLASS_ERROR)?)? {
            let jerror = env.call_method(jstatus, "getError", "(java/lang/Exception)L;", &[])?.l()?;
            let jexception = ContextedGlobal::from_local(env, jerror)?;
            let error = TesseractAndroidError::Gllobal(GlobalError::Exception(jexception));
            Ok(Self::Error(error.into()))
        } else {
            let jexception = Exception::new(env, Some("Invalid object is trying to be converted to tesseract::client::transport::Status"))?;
            env.throw(jexception)?;
            Result::Err(Error::JavaException)
        }
    }
}