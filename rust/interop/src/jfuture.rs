//===------------ jfuture.rs --------------------------------------------===//
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

use jni::errors::{Error, Result};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::{Context, Poll};

use super::future::completion_stage::JCompletionStage;
use super::contexted_global::ContextedGlobal;

pub struct JFuture {
    result: Arc<Mutex<Option<Result<ContextedGlobal>>>>,
    stage: Option<ContextedGlobal>,
}

impl JFuture {
    pub fn from_stage(stage: JCompletionStage) -> Self {
        Self {
            stage: Some(stage.into()),
            result: Arc::new(Mutex::new(None)),
        }
    }

    pub fn failed(error: Error) -> Self {
        Self {
            stage: None,
            result: Arc::new(Mutex::new(Some(Err(error)))),
        }
    }

    pub fn from_stage_result(result: Result<JCompletionStage>) -> Self {
        match result {
            Ok(stage) => Self::from_stage(stage),
            Err(err) => Self::failed(err),
        }
    }
}

impl Future for JFuture {
    type Output = Result<ContextedGlobal>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as Future>::Output> {
        debug!("!@#$%POLLPOLLPOLL");
        let mutex = Arc::clone(&self.result);
        let mut guard = mutex.lock().unwrap();

        debug!("!@#$%GUARD");

        let result = guard.take();

        debug!("!@#$%GUARD2");

        match result {
            None => {
                debug!("!@#$%NONE");
                let waker = cx.waker().clone();

                match &self.stage {
                    Some(stage) => {
                        match stage.local_env() {
                            Ok((env, local)) => {
                                let jstage = JCompletionStage::from_env(&env, local);
                                debug!("!@#$%STAGE");
                                jstage
                                    .when_complete(|env, result| {
                                        let mut guard = mutex.lock().unwrap();

                                        let result = result
                                            .and_then(|r| ContextedGlobal::from_local(&env, r));
                                        *guard = Some(result);
                                        debug!("!@#$%WAKERWAKE");
                                        waker.clone().wake();
                                        debug!("!@#$%WAKERWOKEN");
                                    })
                                    .unwrap(); //TODO: return error
                                debug!("!@#$%PENDING");
                                Poll::Pending
                            }
                            Err(err) => {
                                debug!("!@#$%ERRRRRRR");
                                Poll::Ready(Err(err))
                            }
                        }
                    }
                    None => Poll::Ready(Err(Error::NullDeref(
                        "JFuture was created with no stage and no error. Please, report a bug.",
                    ))),
                }
            }
            Some(result) => {
                debug!("!@#$%SOME");
                Poll::Ready(result)
            }
        }
    }
}
