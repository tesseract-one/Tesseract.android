use std::{sync::Arc, collections::HashMap};

use async_trait::async_trait;

use tesseract_one::client::{Delegate, transport::Status};

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

        let result = match status {
            Status::Ready => Ok(Some(tid)),
            Status::Unavailable(reason) => {
                Err(format!("Transport '{}' is not available because of the following reason: {}", tid, reason))
            },
            Status::Error(e) => {
                Err(format!("Transport '{}' is not available because the transport produced an error: {}", tid, e.to_string()))
            },
        };

        result.unwrap_or_else(|e| {
            self.application.show_alert(&e).unwrap();
            None
        })
    }
}