use crabdroid::future::JCompletionStage;
use jni::objects::{JObject, JString};
use jni::JNIEnv;

use jni_fn::jni_fn;

use crabdroid::object::JavaWrapper;
use crabdroid::error::JavaErrorContext;

use tesseract::client::Service;
use tesseract_android_base::TesseractAndroidError;
use tesseract_protocol_test::{Test, TestService};

use super::JavaProtocol;

impl JavaProtocol for Test {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/client/protocol/TestService"
    }
}

#[jni_fn("one.tesseract.client.protocol.TestService")]
pub fn signTransaction<'a>(env: JNIEnv<'a>, this: JObject<'a>, transaction: JString<'a>) -> JObject<'a> { //CompletionStage<String>
    TesseractAndroidError::java_context(&env, || {
        let service = JavaWrapper::from_java_ref::<dyn Service<Protocol = Test>>(this, &env)?;
        let transaction: String = env.get_string(transaction)?.into();

        JCompletionStage::launch_async(&env, async move |vm| {
            let signed = service.sign_transaction(&transaction).await?;
    
            let env = vm.get_env()?;
            let signed = env.new_string(&signed)?;
    
            Ok(env.new_global_ref(signed)?)
        })
    })
}