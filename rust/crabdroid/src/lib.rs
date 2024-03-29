//===------------ lib.rs --------------------------------------------===//
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

#![feature(iterator_try_collect)]
#![feature(auto_traits, negative_impls)]

#![feature(result_option_inspect)]

#[macro_use]
extern crate log;

#[cfg(feature = "thread-pool")]
pub mod thread_pool;

pub mod bi_consumer;
mod contexted_global;
pub mod env;
pub mod error;
mod exception;
mod jfuture;
pub mod pointer;
pub mod future;
pub mod object;
pub mod collection;
pub mod iter;

pub use contexted_global::ContextedGlobal;
pub use exception::Exception;
pub use jfuture::JFuture;

pub use object::{JavaDesc, JavaWrappableDesc, JavaWrappable, JavaConvertibleDesc, JavaConvertible};
pub use error::JavaErrorContext;
