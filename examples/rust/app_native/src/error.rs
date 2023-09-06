use thiserror::Error;

#[derive(Error, Debug)]
pub (crate) enum Error {
    #[error("JNI error")]
    JNI(#[from] jni::errors::Error),

    #[error("Tesseract error: {0}")]
    Tesseract(#[from] tesseract::Error),
}