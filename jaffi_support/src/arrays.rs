// Copyright 2022 Benjamin Fry <benjaminfry@me.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use jni::objects::{AutoElements, JByteBuffer, JByteArray};

use super::*;

/// Arrays
///
/// If greater than 1 dimension of
///
/// # Type Parameters
///
/// * `N` - The number of dimensions in the array
#[derive(Debug)]
#[repr(transparent)]
pub struct UnsupportedArray<'j>(pub JObject<'j>);

/// Arrays
///
/// If greater than 1 dimension of
///
/// # Type Parameters
///
/// * `N` - The number of dimensions in the array
#[derive(Debug)]
#[repr(transparent)]
pub struct JavaByteArray<'j>(JByteArray<'j>);

impl<'j> JavaByteArray<'j> {
    /// Creates a new array from containing the data from `from`
    pub fn new(env: JNIEnv<'j>, from: &[u8]) -> Result<Self, jni::errors::Error> {
        env.byte_array_from_slice(from)
            .map(|jarray| Self(jarray))
    }

    /// A read-only wrapper around the java array
    pub fn as_slice<'s>(
        &'s self,
        env: &'s mut JNIEnv<'j>,
    ) -> Result<JavaByteArrayRef<'s>, jni::errors::Error> {
        unsafe {
            env.get_array_elements(&self.0, jni::objects::ReleaseMode::NoCopyBack)
        }.map(JavaByteArrayRef)
    }
}

/// Rather than implementing any conversions, the ByteArrays allow present low level options to make the best decision for performance
impl<'j> FromJavaToRust<'j, Self> for JavaByteArray<'j> {
    fn java_to_rust(java: Self, _env: &mut JNIEnv<'j>) -> Self {
        java
    }
}

/// Rather than implementing any conversions, the ByteArrays allow present low level options to make the best decision for performance
impl<'j> FromRustToJava<'j, Self> for JavaByteArray<'j> {
    fn rust_to_java(rust: Self, _env: JNIEnv<'j>) -> Self {
        rust
    }
}

impl<'j> From<JObject<'j>> for JavaByteArray<'j> {
    fn from(jobject: JObject<'j>) -> Self {
        Self(JByteArray::from(jobject))
    }
}

impl<'j> From<JavaByteArray<'j>> for JObject<'j> {
    fn from(jarray: JavaByteArray<'j>) -> Self {
        jarray.0.into()
    }
}

impl<'j> Deref for JavaByteArray<'j> {
    type Target = JObject<'j>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct JavaByteArrayRef<'s>(AutoElements<'s, 's, 's, jni::sys::jbyte>);

impl<'s: 'j, 'j> Deref for JavaByteArrayRef<'s> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        let len = self.0.len();
        let data = self.0.as_ptr() as *const u8;

        unsafe { std::slice::from_raw_parts(data, len) }
    }
}

// ByteBuffer support

/// Rather than implementing any conversions, the ByteArrays allow present low level options to make the best decision for performance
impl<'j> FromJavaToRust<'j, Self> for JByteBuffer<'j> {
    fn java_to_rust(java: Self, _env: &mut JNIEnv<'j>) -> Self {
        java
    }
}

/// Rather than implementing any conversions, the ByteArrays allow present low level options to make the best decision for performance
impl<'j> FromRustToJava<'j, Self> for JByteBuffer<'j> {
    fn rust_to_java(rust: Self, _env: JNIEnv<'j>) -> Self {
        rust
    }
}
