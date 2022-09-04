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

use tesseract::{Error, ErrorKind};
use tesseract_protocol_test::Test;

use super::ui::UI;

pub(crate) struct TestService {
    ui: UI
}

impl TestService {
    pub fn new(ui: UI) -> Self {
        Self { ui: ui }
    }
}

impl tesseract::service::Service for TestService {
    type Protocol = Test;

    fn protocol(&self) -> &Test {
        &Test::Protocol
    }

    fn to_executor(self) -> Box<dyn tesseract::service::Executor + Send + Sync> {
        Box::new(tesseract_protocol_test::service::TestExecutor::from_service(
            self,
        ))
    }
}

#[async_trait]
impl tesseract_protocol_test::TestService for TestService {
    async fn sign_transaction(self: Arc<Self>, req: &str) -> tesseract::Result<String> {
        let allow = self.ui.request_user_confirmation(req).await?;

        if allow {
            if req == "make_error" {
                Err(Error::described(
                    ErrorKind::Weird,
                    "intentional error for test",
                ))
            } else {
                Ok(format!("{}_signed!", req))
            }
        } else {
            Err(tesseract::Error::kinded(tesseract::ErrorKind::Cancelled))
        }

        // let allow = self.core.do_in_context_rret(64, |env, core| {
        //     let core = RustCore::from_env(&env, core);

        //     let stage = core.request_user_confirmation(req);
        //     Ok(JFuture::from_stage_result(stage))
        //     // if req == "make_error" {
        //     //     Err(Error::described(
        //     //         ErrorKind::Weird,
        //     //         "intentional error for test",
        //     //     ))
        //     // } else {
        //     //     Ok(format!("{}_signed!", req))
        //     // }
            
        // }).unwrap().await;

        //let allow.await

        //Ok(format!("{}_signed!", req))
    }
}