use jni::JNIEnv;

use tesseract::Error as TError;

pub fn tesseractify_no_exception<T, I, F>(fun: F) -> tesseract::Result<T>
where
    I: Into<T>,
    F: FnOnce() -> jni::errors::Result<I>,
{
    match fun() {
        Err(err) => {
            Err(TError::new(tesseract::error::ErrorKind::Weird, "JNI Error", err))
        }
        Ok(value) => Ok(value.into())
    }
}

pub fn tesseractify<T, I, F>(env: &JNIEnv, fun: F) -> tesseract::Result<T>
where
    I: Into<T>,
    F: FnOnce() -> jni::errors::Result<I>,
{
    match fun() {
        Err(jni::errors::Error::JavaException) => {
            let (is_cancelled, message) = tesseractify(env, || {
                let exception = env.exception_occurred()?;
                env.exception_clear()?;
                let is_cancelled = env.is_instance_of(exception, "one.tesseract.UserCancelledException")?;
                let message = env.call_method(exception, "getMessage", "()Ljava/lang/String;", &[])?.l()?;
                let message: String = env.get_string(message.into())?.into();
                Ok((is_cancelled, message))
            })?;

            let kind = if is_cancelled {
                tesseract::ErrorKind::Cancelled
            } else {
                tesseract::ErrorKind::Weird
            };

            Err(TError::described(kind, &message))
        }
        other => {
            tesseractify_no_exception(|| {other})
        }
    }
}