use std::{sync::Arc, collections::HashMap};

use async_trait::async_trait;

use tesseract::client::{Delegate, transport::Status};

pub struct TransportDelegate {}

impl TransportDelegate {
    pub fn arc() -> Arc<Self> {
        Arc::new(Self {})
    }
}

#[async_trait]
impl Delegate for TransportDelegate {
    async fn select_transport(
        &self,
        transports: &HashMap<String, Status>,
    ) -> Option<String> {
        assert_eq!(1, transports.len(), "How the heck do we have more than one transport here?");
        let tid = transports.keys().next().map(String::clone).unwrap();

        let status = &transports[&tid];

        match status {
            Status::Ready => Some(tid),
            Status::Unavailable(reason) => {
                debug!("Transport '{}' is not available, because of: {}", tid, reason);
                None
            },
            Status::Error(e) => {
                debug!("Transport '{}' is not available, because the transport produced an error: {}", tid, e.to_string());
                None
            },
        }
    }
}