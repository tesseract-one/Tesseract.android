mod test;
mod substrate;

use std::sync::Arc;

use crabdroid::object::JavaWrapper;
use jni::{objects::JObject, errors::Result, JNIEnv};

use tesseract_one::{
    client::{Tesseract, Service},
    Protocol
};
use tesseract_protocol_substrate::Substrate;
use tesseract_protocol_test::Test;

trait JavaProtocol {
    fn java_class<'a>(&'a self) -> &'a str;
}

fn java_service<'a: 'b, 'b, P: Protocol + JavaProtocol + Copy + 'static>(env: &'b JNIEnv<'a>, tesseract: &Tesseract, protocol: P) -> Result<JObject<'a>> {
    let clazz = protocol.java_class();
    let service = tesseract.service(protocol);
    let service: Arc<dyn Service<Protocol = P>> = service;
    JavaWrapper::java_ref(service, clazz, env)
}

pub(super) fn java_service_by_name<'a: 'b, 'b>(env: &'b JNIEnv<'a>, tesseract: &Tesseract, name: &str) -> Result<JObject<'a>> {
    match name {
        "TestService" => java_service(env, tesseract, Test::Protocol),
        "SubstrateService" => java_service(env, tesseract, Substrate::Protocol),
        _ => panic!("Use one of the designated interfaces from 'one.tesseract.protocol' package")
    }
}