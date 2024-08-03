// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::error::Error;
use crate::read_state::ReadState;
use crate::serializer::Serializer;
use crate::types::{FieldType, FuryGeneralList};
use crate::write_state::WriteState;

#[allow(dead_code)]
fn to_u8_slice<T>(slice: &[T]) -> &[u8] {
    let byte_len = std::mem::size_of_val(slice);
    unsafe { std::slice::from_raw_parts(slice.as_ptr().cast::<u8>(), byte_len) }
}

macro_rules! impl_num_serializer {
    ($name: ident, $ty:tt, $field_type: expr) => {
        impl Serializer for $ty {
            fn write(&self, serializer: &mut WriteState) {
                serializer.writer.$name(*self);
            }

            fn read(deserializer: &mut ReadState) -> Result<Self, Error> {
                Ok(deserializer.reader.$name())
            }

            fn reserved_space() -> usize {
                std::mem::size_of::<$ty>()
            }

            fn ty() -> FieldType {
                $field_type
            }
        }
    };
}
impl FuryGeneralList for i8 {}
impl FuryGeneralList for u16 {}
impl FuryGeneralList for u32 {}
impl FuryGeneralList for u64 {}

impl_num_serializer!(i8, i8, FieldType::INT8);
impl_num_serializer!(u8, u8, FieldType::UINT8);
impl_num_serializer!(i16, i16, FieldType::INT16);
impl_num_serializer!(u16, u16, FieldType::UINT16);
impl_num_serializer!(i32, i32, FieldType::INT32);
impl_num_serializer!(u32, u32, FieldType::UINT32);
impl_num_serializer!(u64, u64, FieldType::UINT64);
impl_num_serializer!(i64, i64, FieldType::INT64);
impl_num_serializer!(f32, f32, FieldType::FLOAT);
impl_num_serializer!(f64, f64, FieldType::DOUBLE);
