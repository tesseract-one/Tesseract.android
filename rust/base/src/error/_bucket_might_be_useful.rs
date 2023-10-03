use jni::{JNIEnv, objects::JObject};

use interop_android::ContextedGlobal;

use tesseract::Result as TResult;
use jni::errors::Result as JResult;

//use crate::error::{tesseractify, tesseractify_no_exception};

/*pub trait TesseractContext {
    fn do_in_tesseract_context<F, R>(&self, capacity: i32, fun: F) -> TResult<R>
        where F: FnOnce(JNIEnv, JObject) -> JResult<R>;
}

impl TesseractContext for ContextedGlobal {
    fn do_in_tesseract_context<F, R>(&self, capacity: i32, fun: F) -> TResult<R>
        where F: FnOnce(JNIEnv, JObject) -> JResult<R> {
            let result: JResult<TResult<R>> = self.do_in_context_rret(capacity, |env, object| {
                Ok(tesseractify(&env, || {
                    fun(env, object)
                }))
            });

            tesseractify_no_exception(|| {result})?
    }
}*/