use jaffi_support::{
    jni::{objects::JObject, JNIEnv},
    Error,
};

use crate::android_annotation::*;

mod android_annotation {
    #![allow(
        dead_code,
        clippy::unused_unit,
        clippy::needless_lifetimes,
        clippy::let_unit_value,
        clippy::let_and_return
    )]

    include!(concat!(env!("OUT_DIR"), "/generated_jaffi.rs"));
}

