#![feature(async_closure)]

mod init;
mod utils;

// #[cfg(feature = "client")]
// pub mod client;

#[cfg(feature = "service")]
pub mod service;