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

use jni::errors::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::{Context, Poll};

use crate::error::GlobalError;
use crate::error::GlobalResult;
use crate::error::LocalError;

use super::future::completion_stage::JCompletionStage;
use super::contexted_global::ContextedGlobal;

pub struct JFuture {
    result: Arc<Mutex<Option<GlobalResult<ContextedGlobal>>>>,
    stage: Option<ContextedGlobal>,
}

impl JFuture {
    pub fn from_stage(stage: JCompletionStage) -> Self {
        Self {
            stage: Some(stage.into()),
            result: Arc::new(Mutex::new(None)),
        }
    }

    pub fn failed(error: GlobalError) -> Self {
        Self {
            stage: None,
            result: Arc::new(Mutex::new(Some(Err(error)))),
        }
    }

    pub fn from_stage_result(result: GlobalResult<JCompletionStage>) -> Self {
        match result {
            Ok(stage) => Self::from_stage(stage),
            Err(err) => Self::failed(err),
        }
    }
}

impl Future for JFuture {
    type Output = GlobalResult<ContextedGlobal>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as Future>::Output> {
        debug!("!@#$%POLLPOLLPOLL");
        
        //the guard needs to be scoped and droped right after the value is taken
        let result = {
            let mutex = Arc::clone(&self.result);
            let mut guard = mutex.lock().unwrap();
            guard.take()
        };

        let result2 = Arc::clone(&self.result);

        debug!("!@#$%GUARD");

        match result {
            None => {
                debug!("!@#$%NONE");
                let waker = cx.waker().clone();

                match &self.stage {
                    Some(stage) => {
                        let mutex = Arc::clone(&self.result);

                        let res = stage.do_in_context_rret(64, move |env, stage| {
                            let jstage = JCompletionStage::from_env(&env, stage);
                            debug!("!@#$%STAGE");

                            let syncronizer: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
                            let syncronizer_when: Arc<Mutex<bool>> = Arc::clone(&syncronizer);
                            // let result_inner = Arc::clone(&result2);

                            // let result2: Arc<Mutex<Option<GlobalResult<ContextedGlobal>>>> = Arc::new(Mutex::new(None));
                            // let result_inner = Arc::clone(&result2);

                            jstage.when_complete(move |env, result| {
                                debug!("!@#$%GUARD1");
                                //the guard needs to be scoped and dropped before waker.wake() call
                                {
                                    let mut guard = mutex.lock().unwrap();
                                    debug!("!@#$%GUARD2");
    
                                    let result = result
                                        .and_then(|r| ContextedGlobal::from_local(&env, r).map_err(|e| {
                                            LocalError::JniError(e)
                                        }) ).map_err(|e| {
                                            e.into_global(&env)
                                        });
                                    *guard = Some(result);
                                    drop(guard);
                                }
                                debug!("!@#$%WAKERWAKE");

                                let synchronizer_guard = syncronizer_when.lock().unwrap();
                                debug!("!@#$%WAKERWAKE_SYNC_GUARD");
                                if *synchronizer_guard {
                                    waker.clone().wake();
                                    debug!("!@#$%WAKERWOKEN");
                                }
                                debug!("!@#$%WAKERWAKE_SYNC_EXITING");
                            })?;
                            debug!("!@#$%PENDING");

                            let result = {
                                let mutex = result2;
                                let mut guard = mutex.lock().unwrap();
                                guard.take()
                            };

                            let result = match result {
                                Some(result) => Poll::Ready(result),
                                None => Poll::Pending,
                            };

                            let mut synchronizer_guard = syncronizer.lock().unwrap();
                            *synchronizer_guard = true;
                            drop(synchronizer_guard); //make it obvious

                            Ok(result)
                        });
                        
                        res.map_err(|e| GlobalError::JniError(e)).unwrap_or_else(|err| {
                            debug!("!@#$%ERRRRRRR");
                            Poll::Ready(Err(err))
                        })
                    }
                    None => Poll::Ready(Err(GlobalError::JniError(Error::NullDeref(
                        "JFuture was created with no stage and no error. Please, report a bug.",
                    )))),
                }
            }
            Some(result) => {
                debug!("!@#$%SOME");
                Poll::Ready(result)
            }
        }
    }
}
