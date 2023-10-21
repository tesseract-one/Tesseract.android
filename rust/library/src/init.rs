use jni::{JNIEnv, objects::JClass};

use jni_fn::jni_fn;

use crabdroid::JavaErrorContext;

use tesseract_android_base::TesseractAndroidError;

#[jni_fn("one.tesseract.TesseractCommon")]
pub fn runtimeInit<'a>(env: JNIEnv<'a>, _: JClass<'a>) {
    TesseractAndroidError::java_context(&env, || {
        let log_level = if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Error
        };
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log_level)
                .with_tag("TESSERACT"),
        );
        
        log_panics::Config::new()
            .backtrace_mode(log_panics::BacktraceMode::Resolved)
            .install_panic_hook();
        
        Ok(())
    })
}