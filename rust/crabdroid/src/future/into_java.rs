use std::{
    pin::Pin,
    future::Future,
    result::Result,
    sync::{Arc, Mutex, MutexGuard},
    task::{Wake, Context, Poll},
};

use jni::{
    JNIEnv,
    objects::GlobalRef
};

use crate::{contexted_global::ContextedGlobal, error::ExceptionConvertible, Exception};

use super::completable_future::JCompletableFuture;

struct Waker<E, F> where
    E: Send + ExceptionConvertible + 'static,
    F: ?Sized + Send + 'static,
    F: Future<Output = Result<GlobalRef, E>> {
        rs: Mutex<Pin<Box<F>>>,
        j: ContextedGlobal
}

impl<E, F> Waker<E, F> where
    E: Send + ExceptionConvertible,
    F: ?Sized + Send,
    F: Future<Output = Result<GlobalRef, E>> {

    pub fn new(rs: Pin<Box<F>>, j: ContextedGlobal) -> Self {
        Self { rs: Mutex::new(rs), j: j }
    }

    fn guard(&self) -> MutexGuard<Pin<Box<F>>> {
        self.rs.lock().unwrap()
    }
}

impl<E, F> Wake for Waker<E, F> where
    E: Send + ExceptionConvertible,
    F: ?Sized + Send,
    F: Future<Output = Result<GlobalRef, E>> {

    fn wake(self: Arc<Self>) {
        self.j.with_safe_context_rret(64, |env, object| {
            let jfut = JCompletableFuture::from_env(&env, object);
            debug!("GOT JFUT");

            let poll = {
                let mut fguard = self.guard();
                debug!("GOT GUARD");
                let waker = Arc::clone(&self).into();
                debug!("WAKER CLONED");
                let mut context = Context::from_waker(&waker);
                debug!("CONTEXT");
                fguard.as_mut().poll(&mut context)
            };
            debug!("POLLED");

            let resolved = match poll {
                Poll::Pending => {true}
                Poll::Ready(r) => {jfut.resolve3(r).inspect_err(|e| {
                    debug!("WTF: {}", e);
                    let exc = e.to_exception(&env).unwrap();
                    let exc = Exception::from_env(&env, exc);
                    exc.print_stack_trace().unwrap();

                    debug!("EXCEPTION PRINTED");
                })?}
            };

            debug!("RESOLVED {}", resolved);

            if !resolved {
                panic!("It's a bug in wake. Why is the resolved future gets resolved again?")
            };

            debug!("NO PANIC");

            Ok(())
        }).unwrap();
    }
}

pub trait IntoJava {
    fn into_java<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> jni::errors::Result<JCompletableFuture<'a, 'b>> where Self: Sized;
    fn boxed_into_java<'a: 'b, 'b>(self: Box<Self>, env: &'b JNIEnv<'a>) -> jni::errors::Result<JCompletableFuture<'a, 'b>>;
    fn pinned_into_java<'a: 'b, 'b>(self: Pin<Box<Self>>, env: &'b JNIEnv<'a>) -> jni::errors::Result<JCompletableFuture<'a, 'b>>;
}

impl<E, F> IntoJava for F where
    E: Send + ExceptionConvertible + 'static,
    F: ?Sized + Send + 'static,
    F: Future<Output = Result<GlobalRef, E>> {

    fn pinned_into_java<'a: 'b, 'b>(self: Pin<Box<Self>>, env: &'b JNIEnv<'a>) -> jni::errors::Result<JCompletableFuture<'a, 'b>> {
        let ljfut = JCompletableFuture::new(env)?;
        let gjfut = ContextedGlobal::from_local(env, *ljfut)?;

        let waker = Arc::new(Waker::new(self, gjfut));
        let poll = {
            let mut fguard = waker.guard();
        
            let waker = Arc::clone(&waker).into();
            let mut context = Context::from_waker(&waker);

            debug!("POLLJUSTPOLL");

            fguard.as_mut().poll(&mut context)
        };
        debug!("POLLJUSTPOLLEND");

        let resolved = match poll {
            Poll::Pending => {true}
            Poll::Ready(r) => {ljfut.resolve3(r)?}
        };

        if !resolved {
            panic!("It's a bug. Why is the resolved future gets resolved again?")
        }

        Ok(ljfut)
    }

    fn into_java<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> jni::errors::Result<JCompletableFuture<'a, 'b>> where Self: Sized {
        Box::pin(self).pinned_into_java(env)
    }

    fn boxed_into_java<'a: 'b, 'b>(self: Box<Self>, env: &'b JNIEnv<'a>) -> jni::errors::Result<JCompletableFuture<'a, 'b>> {
        unsafe { Pin::new_unchecked(self) }.pinned_into_java(env)
    }
}