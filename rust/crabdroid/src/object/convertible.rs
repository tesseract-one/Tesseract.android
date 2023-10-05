//===------------ convertible.rs --------------------------------------------===//
//  Copyright 2022, Tesseract Systems, Inc.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//===----------------------------------------------------------------------===//

use crate::env::AndroidEnv;
use jni::{JNIEnv, objects::{JObject, JValue}, errors::Result};

pub trait JavaConvertibleDesc: Sized {
    fn java_class<'a>(&'a self) -> &'a str;

    fn fields() -> Vec<(&'static str, &'static str)>; //(name, type)

    fn into_values<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<Vec<JValue<'a>>>;
    fn from_values<'a: 'b, 'b>(env: &'b JNIEnv<'a>, values: &[JValue<'a>]) -> Result<Self>;
}

pub trait JavaConvertible: Sized {
    fn into_java<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<JObject<'a>>;
    fn from_java<'a: 'b, 'b>(env: &'b JNIEnv<'a>, object: JObject<'a>) -> Result<Self>;
}

impl<T> JavaConvertible for T where T: JavaConvertibleDesc {
    fn into_java<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<JObject<'a>> {
        let clazz = env.find_class_android(self.java_class())?;

        let types: Vec<&str> = Self::fields().into_iter().map(|field| field.1).collect();
        let sig = format!("({})V", types.join(""));

        let value = self.into_values(env)?;

        env.new_object(clazz, sig, &value)
    }

    fn from_java<'a: 'b, 'b>(env: &'b JNIEnv<'a>, object: JObject<'a>) -> Result<Self> {
        fn uppercase_first_letter(s: &str) -> String {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        }

        fn getter_prefix(ret: &str) -> &'static str {
            let mut c = ret.chars();
            match c.next() {
                None => panic!("Put a sig"),
                Some(f) => {
                    match f {
                        'Z' => "is",
                        _ => "get"
                    }
                }
            }
        }

        let fields = Self::fields();

        let values : Vec<JValue> = fields.into_iter().map(|(field_name, ret)| {
            let getter = format!("{}{}", getter_prefix(ret), uppercase_first_letter(field_name));
            let sig = format!("(){}", ret);

            env.call_method(object, &getter, sig, &[])
        }).try_collect()?;

        Self::from_values(env, &values)
    }
}