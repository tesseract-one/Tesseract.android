use tesseract_protocol_substrate::Substrate;

use super::JavaProtocol;

impl JavaProtocol for Substrate {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/client/protocol/SubstrateService"
    }
}