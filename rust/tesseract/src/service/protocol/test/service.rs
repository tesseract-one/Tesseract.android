use std::sync::Arc;

use async_trait::async_trait;

use interop_android::{ContextedGlobal, JFuture};

use jni::{JNIEnv, objects::JObject, errors::Result};
use tesseract_protocol_test::{Test, service::TestExecutor};

use crate::error::tesseractify_global_result;

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
            let req = env.new_string(req)?;
            let signed = jservice.sign_transaction(req);

            Ok(JFuture::from_stage_result(signed))
        })?.await;

        tesseractify_global_result(global_result)?
            .do_in_tesseract_context(32, |env, signature| {
                Ok(env.get_string(signature.into())?.into())
            })
    }
}