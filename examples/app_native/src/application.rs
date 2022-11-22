use jni::{JNIEnv, objects::JObject};
use jni::errors::Result;

use interop_android::ContextedGlobal;

pub (crate) struct Application {
    application: ContextedGlobal
}

impl Application {
    pub (crate) fn from_env(env: &JNIEnv, application: JObject) -> Result<Self> {
        Ok(Self {
            application: ContextedGlobal::from_local(env, application)?
        })
    }

    pub (crate) fn show_alert(&self, message: &str) -> Result<()> {
        self.application.do_in_context_rret(10, |env, application| {
            let message = env.new_string(message)?;
            env.call_method(application, "showAlert", "(Ljava/lang/String;)V", &[message.into()])?.v()
        })
    }
}