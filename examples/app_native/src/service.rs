//===------------ service.rs --------------------------------------------===//
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

use std::sync::Arc;

use async_trait::async_trait;

use serde::{Deserialize, Serialize};

use tesseract::Protocol;
use tesseract::Result;
use tesseract_client::ErasedService;
use tesseract_client::Service;

pub enum Polkadot {
    Network,
}

impl Protocol for Polkadot {
    fn id(&self) -> String {
        "polkadot".to_owned()
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignTransactionRequest {
    transaction: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignTransactionResponse {
    signed: String,
}

#[async_trait]
pub trait PolkadotService {
    //test method
    async fn sign_transaction(self: Arc<Self>, transaction: &str) -> Result<String>;
}

//potentially, such implementations could be done with code generation in the future
#[async_trait]
impl<T> PolkadotService for T
where
    T: Service<Protocol = Polkadot> + ErasedService + ?Sized,
{
    async fn sign_transaction(self: Arc<Self>, transaction: &str) -> Result<String> {
        let request = SignTransactionRequest {
            transaction: transaction.to_owned(),
        };

        let response: SignTransactionResponse =
            self.call("sign_transaction".to_owned(), request).await?;

        Ok(response.signed)
    }
}
