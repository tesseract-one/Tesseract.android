use std::sync::Arc;

use async_trait::async_trait;

use interop_android::{ContextedGlobal, future::completion_stage::JCompletionStage, JFuture};

use jni::{JNIEnv, objects::{JObject, JString, JValue}, errors::Result, signature};
use tesseract_protocol_test::{Test, service::TestExecutor};

use crate::error::{tesseractify, tesseractify_global_result};

pub struct TestService {
    jservice: ContextedGlobal
}

impl TestService {
    pub fn maybe_new<'a: 'b, 'b>(env: &'b JNIEnv<'a>, service: JObject<'a>) -> Result<Option<Self>> {
        if JTestService::is_test_service(env, service)? {
            Ok(Some(TestService {
                jservice: ContextedGlobal::from_local(env, service)?
            }))
        } else {
            Ok(None)
        }
    }
}

impl tesseract::service::Service for TestService {
    type Protocol = Test;

    fn protocol(&self) -> &Test {
        &Test::Protocol
    }

    fn to_executor(self) -> Box<dyn tesseract::service::Executor + Send + Sync> {
        Box::new(TestExecutor::from_service(
            self,
        ))
    }
}

use crate::context::TesseractContext;

#[async_trait]
impl tesseract_protocol_test::TestService for TestService {
    async fn sign_transaction(self: Arc<Self>, req: &str) -> tesseract::Result<String> {
        let global_result = self.jservice.do_in_tesseract_context(32, |env, jservice| {
            let jservice = JTestService::from_env(&env, jservice);
            let signed = jservice.sign_transaction(req);

            Ok(JFuture::from_stage_result(signed))
        })?.await;

        tesseractify_global_result(global_result)?
            .do_in_tesseract_context(32, |env, signature| {
                Ok(env.get_string(signature.into())?.into())
            })
    }
}

/// Lifetime'd representation of a `JTestService`. Just a `JObject` wrapped in a
/// new class.
#[derive(Clone, Copy)]
struct JTestService<'a: 'b, 'b> {
    internal: JObject<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> ::std::ops::Deref for JTestService<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a: 'b, 'b> From<JTestService<'a, 'b>> for JObject<'a> {
    fn from(other: JTestService<'a, 'b>) -> JObject<'a> {
        other.internal
    }
}

impl<'a: 'b, 'b> JTestService<'a, 'b> {
    fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> JTestService<'a, 'b> {
       JTestService {
            internal: obj,
            env: env,
        }
    }

    fn is_test_service(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> Result<bool> {
        env.is_instance_of(obj, "one.tesseract.service.protocol.TestService")
    }

    fn sign_transaction(&self, request: &str) -> Result<JCompletionStage> {
        // let stage = self.env
        // .call_method(
        //     self.internal,
        //     "requestUserConfirmation",
        //     "(Landroid/os/Parcelable;)Ljava/util/concurrent/CompletionStage;",
        //     &[JValue::from(request)],
        // )?
        // .l()?;

        // Ok(JCompletionStage::from_env(&self.env, stage))
        todo!()
    }
}