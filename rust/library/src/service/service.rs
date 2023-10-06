use jni::{JNIEnv, objects::JObject};
use jni::errors::Result;

use tesseract_android_base::service::Applicator;

use crate::service::protocol;

pub (crate) fn jservice_to_services<'a: 'b, 'b>(env: &'b JNIEnv<'a>, service: JObject<'a>) -> Result<Vec<Box<dyn Applicator>>> {
    let mut result = Vec::<Box<dyn Applicator>>::new();

    #[cfg(feature = "protocol-test")]
    if let Some(test_service) = protocol::TestService::maybe_new(env, service)? {
        result.push(Box::new(|tesseract| {
            tesseract.service(test_service)
        }))
    }

    Ok(result)
}