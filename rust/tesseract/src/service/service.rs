use jni::{JNIEnv, objects::JObject};
use jni::errors::Result;

use crate::service::protocol;
use super::tesseract::Applicator;

pub (crate) fn jservice_to_services<'a: 'b, 'b>(env: &'b JNIEnv<'a>, service: JObject<'a>) -> Result<Vec<Box<dyn Applicator>>> {
    let mut result = Vec::<Box<dyn Applicator>>::new();

    if let Some(test_service) = protocol::TestService::maybe_new(env, service)? {
        result.push(Box::new(|tesseract| {
            tesseract.service(test_service)
        }))
    }

    Ok(result)
}