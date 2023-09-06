use std::{sync::Arc, collections::HashMap};

use async_trait::async_trait;

use tesseract::client::{Delegate, transport::Status};

use crate::application::Application;

pub (crate) struct TransportDelegate {
    application: Application
}

impl TransportDelegate {
    pub (crate) fn arc(application: Application) -> Arc<Self> {
        Arc::new(Self {application: application})
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
                self.application.show_alert(&format!("Transport '{}' is not available because of the following reason: {}", tid, reason)).unwrap();
                None
            },
            Status::Error(e) => {
                self.application.show_alert(&format!("Transport '{}' is not available because the transport produced an error: {}", tid, e.to_string())).unwrap();
                None
            },
        }
    }
}