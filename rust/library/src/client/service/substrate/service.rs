use jni::objects::{JObject, JString};
use jni::JNIEnv;

use jni_fn::jni_fn;

use crabdroid::JavaConvertible;
use crabdroid::future::JCompletionStage;
use crabdroid::object::JavaWrapper;
use crabdroid::error::JavaErrorContext;

use log::debug;
use tesseract::client::Service;
use tesseract_android_base::TesseractAndroidError;

use tesseract_protocol_substrate::{Substrate, SubstrateService, AccountType};

use crate::utils::{FromJava, Wrapper};

use super::super::JavaProtocol;

impl JavaProtocol for Substrate {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/client/protocol/SubstrateService"
    }
}

#[jni_fn("one.tesseract.client.protocol.SubstrateService")]
pub fn getAccount<'a>(env: JNIEnv<'a>, this: JObject<'a>, accountType: JObject<'a>) -> JObject<'a> { //CompletionStage<GetAccountResponse>
    TesseractAndroidError::java_context(&env, || {
        debug!("SubstrateService: getAccount");
        let service = JavaWrapper::from_java_ref::<dyn Service<Protocol = Substrate>>(this, &env)?;
        debug!("SubstrateService: got service");
        let account_type = AccountType::from_java(&env, accountType)?;
        debug!("SubstrateService: converted account type");

        JCompletionStage::launch_async(&env, async move |vm| {
            debug!("SubstrateService: get account launched async");
            let account_response = service.get_account(account_type).await?;
            debug!("SubstrateService: got account with path {}", &account_response.path);

            let env = vm.get_env()?;

            debug!("SubstrateService: got VM");

            let account_response = Wrapper::from(account_response).into_java(&env)?;

            debug!("SubstrateService: converted response into java");

            Ok(env.new_global_ref(account_response)?)
        })
    })
}

#[jni_fn("one.tesseract.client.protocol.SubstrateService")]
pub fn signTransaction<'a>(env: JNIEnv<'a>, this: JObject<'a>,
    accountType: JObject<'a>,
    accountPath: JString<'a>,
    extrinsicData: JObject<'a>,
    extrinsicMetadata: JObject<'a>,
    extrinsicTypes: JObject<'a>) -> JObject<'a> { //CompletionStage<ByteArray>
        TesseractAndroidError::java_context(&env, || {
            let service = JavaWrapper::from_java_ref::<dyn Service<Protocol = Substrate>>(this, &env)?;

            let account_type = AccountType::from_java(&env, accountType)?;
            let account_path: String = env.get_string(accountPath)?.into();
            let extrinsic_data = env.convert_byte_array(extrinsicData.into_raw())?;
            let extrinsic_metadata = env.convert_byte_array(extrinsicMetadata.into_raw())?;
            let extrinsic_types = env.convert_byte_array(extrinsicTypes.into_raw())?;

            JCompletionStage::launch_async(&env, async move |vm| {
                let signature = service.sign_transaction(account_type, &account_path, &extrinsic_data, &extrinsic_metadata, &extrinsic_types).await?;

                let env = vm.get_env()?;
                let signature = env.byte_array_from_slice(&signature)?;

                let signature = unsafe {
                    JObject::from_raw(signature)
                };

                Ok(env.new_global_ref(signature)?)
            })
        })
}