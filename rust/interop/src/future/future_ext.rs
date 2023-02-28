use std::future::Future;
use futures::FutureExt;

use jni::{
    errors::Error as JError,
    objects::{JObject, GlobalRef},
    JNIEnv
};

use std::fmt::Display;

/*
    These extentions can be done with static typing of produced futures, though it requires significantly more work
    so leaving it as is for now.
 */

pub trait FutureExtJava: FutureExt {
    fn map_java<'a: 'b, 'b, E, F>(self, env: &'b JNIEnv<'a>, f: F) -> Box<dyn Future<Output=Result<GlobalRef, E>> + Send + 'static> where
        Self: Sized + Send + 'static,
        E: From<JError> + Send + Display + 'static,
        F: Send + 'static,
        F: for <'k> FnOnce(&'k JNIEnv<'k>, Self::Output) -> Result<JObject<'k>, E> {
            let vm = env.get_java_vm();
            let vm = match vm {
                Ok(vm) => vm,
                Err(err) => return Box::new(futures::future::ready(Err(err.into())))
            };
            Box::new(self.map(move |it| {
                let env = vm.get_env()?;
                let u = f(&env, it)?;
                Ok(env.new_global_ref(u)?)
            }))
        }

    fn map_ok_java<'a: 'b, 'b, T, E, F>(self, env: &'b JNIEnv<'a>, f: F) -> Box<dyn Future<Output=Result<GlobalRef, E>> + Send + 'static> where
        Self: Sized + Send + 'static,
        Self: Future<Output=Result<T,E>>,
        E: From<JError> + Send + Display + 'static,
        F: Send + 'static,
        F: for <'k> FnOnce(&'k JNIEnv<'k>, T) -> Result<JObject<'k>, E> {
            let vm = env.get_java_vm();
            let vm = match vm {
                Ok(vm) => vm,
                Err(err) => return Box::new(futures::future::ready(Err(err.into())))
            };
            Box::new(self.map(move |it| {
                match it {
                    Ok(it) => {
                        let env = vm.get_env()?;
                        let u = f(&env, it)?;
                        Ok(env.new_global_ref(u)?)
                    },
                    Err(err) => Err(err)
                }
            }))
        }
}

impl<F> FutureExtJava for F where F: FutureExt {
}
