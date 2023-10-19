use jni::{JNIEnv, objects::JClass};

use jni_fn::jni_fn;

use crabdroid::JavaErrorContext;

use tesseract_android_base::TesseractAndroidError;

#[jni_fn("one.tesseract.TesseractCommon")]
pub fn runtimeInit<'a>(env: JNIEnv<'a>, _: JClass<'a>) {
    TesseractAndroidError::java_context(&env, || {
        android_log::init("TESSERACT")?;
        log_panics::init();
        Ok(())
    })
}