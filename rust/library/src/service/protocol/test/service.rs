use std::sync::Arc;

use async_trait::async_trait;

use jni::{JNIEnv, objects::JObject, errors::Result};

use crabdroid::ContextedGlobal;

use errorcon::convertible::ErrorContext;
use tesseract_protocol_test::{Test, service::TestExecutor};
use tesseract_android_base::TesseractAndroidError;

use super::jservice::JTestService;

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

impl tesseract_one::service::Service for TestService {
    type Protocol = Test;

    fn protocol(&self) -> &Test {
        &Test::Protocol
    }

    fn to_executor(self) -> Box<dyn tesseract_one::service::Executor + Send + Sync> {
        Box::new(TestExecutor::from_service(
            self,
        ))
    }
}

#[async_trait]
impl tesseract_protocol_test::TestService for TestService {
    async fn sign_transaction(self: Arc<Self>, req: &str) -> tesseract_one::Result<String> {
        TesseractAndroidError::context_async( async || {
            let response = self.jservice.with_async_context(32, |env, jservice| {
                let jservice = JTestService::from_env(&env, jservice);
                let req = env.new_string(req)?;
                jservice.sign_transaction(req)
            }).await?;

            let response = response.with_safe_context_rret(32, |env, signature| {
                let string: String = env.get_string(signature.into())?.into();
                Ok(string)
            })?;

            Ok(response)
        }).await
    }
}