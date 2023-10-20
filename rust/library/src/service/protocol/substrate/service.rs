use std::sync::Arc;

use async_trait::async_trait;

use jni::{JNIEnv, objects::JObject, errors::Result};

use crabdroid::ContextedGlobal;

use errorcon::convertible::ErrorContext;
use tesseract_protocol_substrate::{Substrate, service::SubstrateExecutor, GetAccountResponse, AccountType};
use tesseract_android_base::TesseractAndroidError;

use crate::utils::{IntoJava, Wrapper};

use super::jservice::JSubstrateService;

pub struct SubstrateService {
    jservice: ContextedGlobal
}

impl SubstrateService {
    pub fn maybe_new<'a: 'b, 'b>(env: &'b JNIEnv<'a>, service: JObject<'a>) -> Result<Option<Self>> {
        if JSubstrateService::is_substrate_service(env, service)? {
            Ok(Some(SubstrateService {
                jservice: ContextedGlobal::from_local(env, service)?
            }))
        } else {
            Ok(None)
        }
    }
}

impl tesseract::service::Service for SubstrateService {
    type Protocol = Substrate;

    fn protocol(&self) -> &Substrate {
        &Substrate::Protocol
    }

    fn to_executor(self) -> Box<dyn tesseract::service::Executor + Send + Sync> {
        Box::new(SubstrateExecutor::from_service(
            self,
        ))
    }
}

use crabdroid::JavaConvertible;

#[async_trait]
impl tesseract_protocol_substrate::SubstrateService for SubstrateService {
    async fn get_account(self: Arc<Self>, account_type: AccountType) -> tesseract::Result<GetAccountResponse> {
        TesseractAndroidError::context_async( async || {
            let response = self.jservice.with_async_context(32, |env, jservice| {
                let jservice = JSubstrateService::from_env(&env, jservice);
                let account_type = account_type.into_java(&env)?;
                jservice.get_account(account_type)
            }).await?;

            let response: GetAccountResponse = response.with_safe_context_rret(32, |env, response| {
                Wrapper::from_java(&env, response)
                    .map(|c| c.inner)
            })?;

            Ok(response)
        }).await
    }

    async fn sign_transaction(
        self: Arc<Self>,
        account_type: AccountType,
        account_path: &str,
        extrinsic_data: &[u8],
        extrinsic_metadata: &[u8],
        extrinsic_types: &[u8],
    ) -> tesseract::Result<Vec<u8>> {
        TesseractAndroidError::context_async( async || {
            let response = self.jservice.with_async_context(32, |env, jservice| {
                let jservice = JSubstrateService::from_env(&env, jservice);

                let account_type = account_type.into_java(&env)?;
                let account_path = env.new_string(account_path)?;
                let extrinsic_data = env.byte_array_from_slice(extrinsic_data)?;
                let extrinsic_metadata = env.byte_array_from_slice(extrinsic_metadata)?;
                let extrinsic_types = env.byte_array_from_slice(extrinsic_types)?;

                let (extrinsic_data, extrinsic_metadata, extrinsic_types) = unsafe {
                     (
                        JObject::from_raw(extrinsic_data),
                        JObject::from_raw(extrinsic_metadata),
                        JObject::from_raw(extrinsic_types),
                     )
                };

                jservice.sign_transaction(account_type, account_path, extrinsic_data, extrinsic_metadata, extrinsic_types)
            }).await?;

            let response = response.with_safe_context_rret(32, |env, response| {
                env.convert_byte_array(response.into_raw())
            })?;

            Ok(response)
        }).await
    }
}