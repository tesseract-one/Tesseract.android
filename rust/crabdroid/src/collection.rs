use jni::{
    descriptors::Desc,
    errors::{
        Result as JResult,
        Error as JError
    },
    objects::{JClass, JObject, JList, AutoLocal},
    sys::{jsize, jobjectArray},
    JNIEnv
};

use super::iter::{
    IntoExactSizeIterator,
    FromExactSizeIteratorJava
};

pub trait JListExt<'a: 'b, 'b> {
    unsafe fn from_array(env: &'b JNIEnv<'a>, array: jobjectArray) -> JResult<Self> where Self: Sized;
}

impl<'a: 'b, 'b>  JListExt<'a, 'b> for JList<'a, 'b> {
    unsafe fn from_array(env: &'b JNIEnv<'a>, array: jobjectArray) -> JResult<Self> {
        let object = env.call_static_method(
            env.find_class("java/util/Arrays")?,
            "asList",
            "([Ljava/lang/Object;)Ljava/util/List;",
            &[JObject::from_raw(array).into()],
        )?
        .l()?;

        JList::from_env(env, object)
    }
}

impl<'a: 'b, 'b, T> FromExactSizeIteratorJava<'a, 'b, T> for jobjectArray where T: Into<JObject<'a>> {
    fn from_iter_java<'c, C, E, I>(env: &'b JNIEnv<'a>, iter: I, element_class: C) -> Result<Self, E>
    where
        E: From<JError>,
        C: Desc<'a, JClass<'c>>,
        <I as IntoIterator>::IntoIter: ExactSizeIterator,
        I: IntoExactSizeIterator<Item = Result<T, E>> {
            let iter = iter.into_iter();
            let array = env.new_object_array(
                iter.len() as jsize,
                element_class,
                JObject::null(),
            )?;
            for (i, object) in iter.enumerate() {
                let object = AutoLocal::new(env, object?.into());
                env.set_object_array_element(array, i as jsize, object.as_obj())?;
            }
            Ok(array)
    }
}

impl<'a: 'b, 'b, T> FromExactSizeIteratorJava<'a, 'b, T> for JList<'a, 'b> where T: Into<JObject<'a>> {
    fn from_iter_java<'c, C, E, I>(env: &'b JNIEnv<'a>, iter: I, element_class: C) -> Result<Self, E>
    where
        E: From<JError>,
        C: Desc<'a, JClass<'c>>,
        <I as IntoIterator>::IntoIter: ExactSizeIterator,
        I: IntoExactSizeIterator<Item = Result<T, E>> {
            let array = jobjectArray::from_iter_java(env, iter, element_class)?;
            Ok(unsafe {
                JList::from_array(env, array)
            }?)
        }
}