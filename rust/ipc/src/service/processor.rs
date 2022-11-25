use std::sync::Arc;

use futures::FutureExt;
use jni::objects::JObject;
use jni::JNIEnv;
use jni::errors::Result;

use jni_fn::jni_fn;

use interop_android::pointer::ArcPointer;
use interop_android::env::AndroidEnv;
use interop_android::future::into_java::FutureJava;

use tesseract::service::TransportProcessor;

#[derive(Clone, Copy)]
pub struct JProcessor<'a: 'b, 'b> {
    internal: JObject<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> ::std::ops::Deref for JProcessor<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a: 'b, 'b> From<JProcessor<'a, 'b>> for JObject<'a> {
    fn from(other: JProcessor<'a, 'b>) -> JObject<'a> {
        other.internal
    }
}

impl<'a: 'b, 'b> JProcessor<'a, 'b> {
    fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> JProcessor<'a, 'b> {
        JProcessor {
            internal: obj,
            env: env,
        }
    }

    pub fn new(env: &'b JNIEnv<'a>, processor: Arc<dyn TransportProcessor + Send + Sync>) -> Result<Self> {
        let clazz = env.find_class_android("one/tesseract/ipc/service/TransportProcessor")?;
        let p: i64 = ArcPointer::new(processor).into();

        let obj = env.new_object(clazz, "(J)V", &[p.into()])?;

        Ok(Self::from_env(env, obj))
    }

    fn processor_l(&self) -> Result<i64> {
        self.env
            .call_method(self.internal, "getNative", "()J", &[])?
            .j()
    }

    fn processor(&self) -> Result<Arc<dyn TransportProcessor + Send + Sync>> {
        let processor_l = self.processor_l()?;

        Ok(ArcPointer::of(processor_l).arc())
    }

    fn destroy_rust(&self) -> Result<()> {
        let processor_l = self.processor_l()?;

        let arcp:ArcPointer<dyn TransportProcessor + Send + Sync> = ArcPointer::of(processor_l);

        Ok(arcp.destroy())
    }
}

#[jni_fn("one.tesseract.ipc.service.TransportProcessor")]
pub fn process<'a>(env: JNIEnv<'a>, jprocessor: JObject<'a>, data: jni::sys::jbyteArray) -> JObject<'a> { //returns CompletionStage<ByteArray>
    
    let jprocessor = JProcessor::from_env(&env, jprocessor);
    let processor = jprocessor.processor().unwrap();//TODO: handle error

    let data = env.convert_byte_array(data).unwrap();//TODO: handle error

    let response = async move {
        processor.process(&data).await
    };

    let vm = env.get_java_vm().unwrap();
    let f = response.map(move |data| {
        let env = vm.get_env().unwrap();
        let bytes = env.byte_array_from_slice(&data).unwrap();
        let r = env.new_global_ref(bytes).unwrap();
        return Result::Ok(r);
    });

    *f.into_java(&env)
}

#[jni_fn("one.tesseract.ipc.service.TransportProcessor")]
pub fn finalize(env: JNIEnv, jprocessor: JObject) {
    let jprocessor = JProcessor::from_env(&env, jprocessor);

    jprocessor.destroy_rust().unwrap();

    debug!("!!!!drop jprocessor");
}