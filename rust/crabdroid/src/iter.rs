use jni::{
    descriptors::Desc,
    errors::{
        Result as JResult,
        Error as JError
    },
    objects::{JClass, JObject},
    JNIEnv
};

use std::result::Result;

/*
This module can be done more generic (as try_collect),
though it's still unstable and while it's fine to use complete unstable features,
we'd prefer not to depend on too many unstable implementation intricate details.
*/

pub trait IntoExactSizeIterator: IntoIterator where Self::IntoIter: ExactSizeIterator<Item = Self::Item> {
}

impl<I> IntoExactSizeIterator for I
    where
        I: IntoIterator,
        I::IntoIter: ExactSizeIterator {
}

pub trait FromExactSizeIteratorJava<'a: 'b, 'b, T>: Sized {
    fn from_iter_java<'c, C, E, I>(env: &'b JNIEnv<'a>, iter: I, element_class: C) -> Result<Self, E>
    where
        E: From<JError>,
        C: Desc<'a, JClass<'c>>,
        <I as IntoIterator>::IntoIter: ExactSizeIterator,
        I: IntoExactSizeIterator<Item = Result<T, E>>;
}

pub trait ExactSizeIteratorJava: ExactSizeIterator {
    fn collect_java<'a: 'b, 'b, 'c, C, R: FromExactSizeIteratorJava<'a, 'b, Self::Item>>(self, env: &'b JNIEnv<'a>, element_class: C) -> JResult<R>
    where
        Self: Sized,
        C: Desc<'a, JClass<'c>> {
            let iter = self.map(|it| Ok(it));
            R::from_iter_java(env, iter, element_class)
        }

    fn try_collect_java<'a: 'b, 'b, 'c, C, T, E, R: FromExactSizeIteratorJava<'a, 'b, T>>(self, env: &'b JNIEnv<'a>, element_class: C) -> Result<R, E>
    where
        Self: Sized,
        C: Desc<'a, JClass<'c>>,
        T: Into<JObject<'a>>,
        E: From<JError>,
        Self: ExactSizeIterator<Item = Result<T, E>> {
            R::from_iter_java(env, self, element_class)
        }
}

impl<I> ExactSizeIteratorJava for I where I: ExactSizeIterator {
}