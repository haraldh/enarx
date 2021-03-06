// Copyright 2019 Red Hat, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Intel SGX Documentation is available at the following link.
//! Section references in further documentation refer to this document.
//! https://www.intel.com/content/dam/www/public/emea/xe/en/documents/manuals/64-ia-32-architectures-software-developer-vol-3d-part-4-manual.pdf

#![deny(clippy::all)]
#![allow(clippy::identity_op)]

macro_rules! defenum {
    ($name:ident::$value:ident) => {
        impl Default for $name {
            fn default() -> Self {
                $name::$value
            }
        }
    };
}

macro_rules! defflags {
    ($name:ident $($value:ident)|*) => {
        impl Default for $name {
            fn default() -> Self {
                $name::empty() $( | $name::$value )* | $name::empty()
            }
        }
    };
}

macro_rules! testaso {
    (@off $name:ty=>$field:ident) => {
        &unsafe { &*core::ptr::null::<$name>() }.$field as *const _ as usize
    };

    ($(struct $name:ty: $align:expr, $size:expr => { $($field:ident: $offset:expr),* })+) => {
        #[cfg(test)]
        #[test]
        fn align() {
            use core::mem::align_of;

            $(
                assert_eq!(
                    align_of::<$name>(),
                    $align,
                    "align: {}",
                    stringify!($name)
                );
            )+
        }

        #[cfg(test)]
        #[test]
        fn size() {
            use core::mem::size_of;

            $(
                assert_eq!(
                    size_of::<$name>(),
                    $size,
                    "size: {}",
                    stringify!($name)
                );
            )+
        }

        #[cfg(test)]
        #[test]
        fn offsets() {
            $(
                $(
                    assert_eq!(
                        testaso!(@off $name=>$field),
                        $offset,
                        "offset: {}::{}",
                        stringify!($name),
                        stringify!($field)
                    );
                )*
            )+
        }
    };
}

mod utils;

pub mod attr;
pub mod misc;
pub mod page;
pub mod secs;
pub mod sig;
pub mod ssa;
pub mod tcs;

use core::marker::PhantomData;

#[repr(transparent)]
#[derive(Debug)]
pub struct Offset<T: ?Sized>(u64, PhantomData<T>);

impl<T: ?Sized> From<usize> for Offset<T> {
    fn from(value: usize) -> Self {
        Offset(value as u64, PhantomData)
    }
}
