use std::borrow::BorrowMut;
use std::fmt::Display;
use std::future::Future;
use std::task::{Wake, Context, Poll};
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard};

use jni::JNIEnv;
use jni::objects::{JObject, JThrowable, GlobalRef};

use crate::contexted_global::ContextedGlobal;

use super::completable_future::JCompletableFuture;

struct Waker<E: Send + Display + 'static, F: Future<Output = std::result::Result<GlobalRef, E>> + Send + 'static> {
    rs: Mutex<Pin<Box<F>>>,
    j: ContextedGlobal
}

impl<E: Send + Display, F: Future<Output = std::result::Result<GlobalRef, E>> + Send> Waker<E, F>  {
    pub fn new(rs: Pin<Box<F>>, j: ContextedGlobal) -> Self {
        Self { rs: Mutex::new(rs), j: j }
    }

    fn guard(&self) -> MutexGuard<Pin<Box<F>>> {
        self.rs.lock().unwrap()
    }
}

impl<E: Send + Display, F: Future<Output = std::result::Result<GlobalRef, E>> + Send> Wake for Waker<E, F> {
    fn wake(self: Arc<Self>) {
        let (env, object) = self.j.local_env().unwrap();
        let jfut = JCompletableFuture::from_env(&env, object);

        let mut fguard = self.guard();
        let waker = Arc::clone(&self).into();
        let mut context = Context::from_waker(&waker);
        let poll = fguard.as_mut().poll(&mut context);

        let resolved = match poll {
            Poll::Pending => {false}
            Poll::Ready(r) => {jfut.resolve3(r).unwrap()}
        };

        if !resolved {
            panic!("Wrong future state after the waker is awoken. A bug?")
        }
    }
}

pub trait FutureJava {
    fn into_java<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> JCompletableFuture<'a, 'b>;
}

impl<E: Send + Display + 'static, F: Send + 'static> FutureJava for F where F: Future<Output = std::result::Result<GlobalRef, E>> {
    fn into_java<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> JCompletableFuture<'a, 'b> {
        let ljfut = JCompletableFuture::new(env).unwrap();
        let gjfut = ContextedGlobal::from_local(env, *ljfut).unwrap();

        let boxed = Box::pin(self);
        let waker = Arc::new(Waker::new(boxed, gjfut));
        let mut fguard = waker.guard();
        
        let waker = Arc::clone(&waker).into();
        let mut context = Context::from_waker(&waker);

        let poll = fguard.as_mut().poll(&mut context);

        let resolved = match poll {
            Poll::Pending => {true}
            Poll::Ready(r) => {ljfut.resolve3(r).unwrap()}
        };

        if !resolved {
            panic!("It's a bug. Why is the resolved future gets resolved again?")
        }

        ljfut
    }
}