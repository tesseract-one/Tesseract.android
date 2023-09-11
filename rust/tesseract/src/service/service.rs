use jni::{JNIEnv, objects::JObject};
use jni::errors::Result;
use tesseract::service::Tesseract;

use crate::service::protocol;

pub (crate) trait ServiceApplicator: FnOnce(Tesseract) -> Tesseract {}
impl<F> ServiceApplicator for F where F: FnOnce(Tesseract) -> Tesseract {}

pub (crate) fn jservice_to_services<'a: 'b, 'b>(env: &'b JNIEnv<'a>, service: JObject<'a>) -> Result<Vec<Box<dyn ServiceApplicator>>> {
    let mut result = Vec::<Box<dyn ServiceApplicator>>::new();

    if let Some(test_service) = protocol::TestService::maybe_new(env, service)? {
        result.push(Box::new(|tesseract| {
            tesseract.service(test_service)
        }))
    }

    Ok(result)
}