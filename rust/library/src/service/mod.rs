mod tesseract;
mod service;
mod protocol;

//don't delete. we need this so that the compiler does not optimize it out
pub use tesseract_android_transport::service as ts;