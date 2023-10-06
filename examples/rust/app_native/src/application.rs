use jni::{JNIEnv, objects::JObject};
use jni::errors::Result as JResult;

use crabdroid::ContextedGlobal;

use tesseract_android::error::TesseractAndroidError;

pub (crate) struct Application {
    application: ContextedGlobal
}

impl Application {
    pub (crate) fn from_env(env: &JNIEnv, application: JObject) -> JResult<Self> {
        Ok(Self {
            application: ContextedGlobal::from_local(env, application)?
        })
    }

    pub (crate) fn show_alert(&self, message: &str) -> Result<(), TesseractAndroidError> {
        Ok(self.application.with_safe_context_rret(10, |env, application| {
            let message = env.new_string(message)?;
            env.call_method(application, "showAlert", "(Ljava/lang/String;)V", &[message.into()])?.v()
        })?)
    }
}