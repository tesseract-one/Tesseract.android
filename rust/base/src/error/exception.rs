use jni::{JNIEnv, objects::JObject, errors::Result};

use crabdroid::{error::{GlobalError, ExceptionConvertible}, env::AndroidEnv};

use log::SetLoggerError;

pub fn logger_error_to_exception<'a: 'b, 'b>(error: &SetLoggerError, env: &'b JNIEnv<'a>) -> Result<JObject<'a>> {
    let description = format!("Can't set android logger for Tesseract: {}", error);
    let description = env.new_string(description)?;

    env.new_object(
        "java/lang/Exception",
        "(Ljava/lang/String;)V",
        &[description.into()])
}

pub fn tesseract_error_to_exception<'a: 'b, 'b>(error: &tesseract::Error, env: &'b JNIEnv<'a>) -> Result<JObject<'a>> {
    match &error.kind {
        tesseract::ErrorKind::Cancelled => {
            let clazz = env.find_class_android("one/tesseract/exception/UserCancelledException")?;
            if let Some(description) = &error.description {
                let description = env.new_string(description)?;
                env.new_object(
                    clazz,
                    "(Ljava/lang/String;)V",
                    &[description.into()])
            } else {
                env.new_object(
                    clazz,
                    "()V",
                    &[])
            }
        },
        kind => {
            let kind = kind.to_string();

            let description = if let Some(description) = &error.description {
                format!("Tesseract error with of type '{}': {}", kind, description)
            } else {
                format!("Tesseract error with no description of type: : {}", kind)
            };

            let description = env.new_string(description)?;

            env.new_object(
                "java/lang/Exception",
                "(Ljava/lang/String;)V",
                &[description.into()])
        },
    }
}

pub fn global_error_to_exception<'a: 'b, 'b>(error: &GlobalError, env: &'b JNIEnv<'a>) -> Result<JObject<'a>> {
    match error {
        GlobalError::Exception(error) =>
            error.do_in_context_rret(64, |_, exception| {
                env.new_local_ref(exception)
            }),
        GlobalError::JniError(error) => {
            error.to_exception(&env)
        },
    }
}
