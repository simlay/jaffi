// Copyright 2022 Benjamin Fry <benjaminfry@me.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use jni::objects::JNIString;

pub struct Exception<'j, 'c: 'j, E: Desc<'a, JClass<'c>>, S: 'j + Into<JNIString>> {
    exception: E,
    msg: S,
}
