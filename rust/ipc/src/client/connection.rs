//===------------ connection.rs --------------------------------------------===//
//  Copyright 2022, Tesseract Systems, Inc.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//===----------------------------------------------------------------------===//

use std::collections::LinkedList;
use std::sync::Arc;

use async_trait::async_trait;

use futures::lock::Mutex;

use interop_android::error::GlobalResult;
use tesseract::{Error, ErrorKind, Result, Protocol};
use tesseract::client::Connection;

use interop_android::ContextedGlobal;

use super::response::Response;
use super::transceiver::Transceiver;

pub struct TransportIPCAndroidConnection {
    transceiver: ContextedGlobal,
    responses: Mutex<LinkedList<Response>>,
    protocol: Box<dyn Protocol>
}

impl TransportIPCAndroidConnection {
    pub fn new(transceiver: ContextedGlobal, protocol: Box<dyn Protocol>) -> Self {
        Self {
            transceiver: transceiver,
            responses: Mutex::new(LinkedList::new()),
            protocol: protocol
        }
    }

    async fn send_receive(self: Arc<Self>, request: Vec<u8>) -> GlobalResult<Response> {
        let data = {
            let result = self.transceiver.with_safe_context_rret(64, |env, tran| {
                let transceiver = Transceiver::from_env(&env, tran);
                let result = transceiver.transceive(&request, &self.protocol.id());
                Ok(result)
            })?;
            //yes, it doesn't work shorthand - fucking rust
            result
        }
        .await?;

        let response = {
            data.with_safe_context_rret(64, |env, response| {
                Ok(Response::from_java(&env, response))
            })?
        };

        Ok(response)
    }
}

#[async_trait]
impl Connection for TransportIPCAndroidConnection {
    async fn send(self: Arc<Self>, request: Vec<u8>) -> Result<()> {
        use super::response::Flattener;

        let response = Arc::clone(&self).send_receive(request).await;

        let mut responses = self.responses.lock().await;
        responses.push_back(response.flatten());

        Ok(())
    }

    async fn receive(self: Arc<Self>) -> Result<Vec<u8>> {
        let mut responses = self.responses.lock().await;
        match responses.pop_back() {
            Some(response) => match response {
                Response::Ok(data) => Ok(data),
                Response::Cancelled => Err(Error::kinded(ErrorKind::Cancelled)),
                Response::JniError(error) => {
                    let message = format!("JNI Error just happened: {}", error);
                    Err(Error::described(ErrorKind::Weird, &message))
                }
                Response::Exception(message) => Err(Error::described(ErrorKind::Weird, &message)),
            },
            None => Err(Error::kinded(ErrorKind::Weird)),
        }
    }
}
