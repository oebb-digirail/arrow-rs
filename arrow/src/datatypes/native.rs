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

use crate::error::{ArrowError, Result};
pub use arrow_buffer::{ArrowNativeType, ToByteSlice};
use half::f16;
use num::Zero;

pub use arrow_array::ArrowPrimitiveType;

pub(crate) mod native_op {
    use super::ArrowNativeType;
    use crate::error::{ArrowError, Result};
    use num::Zero;
    use std::ops::{Add, Div, Mul, Sub};

    /// Trait for ArrowNativeType to provide overflow-checking and non-overflow-checking
    /// variants for arithmetic operations. For floating point types, this provides some
    /// default implementations. Integer types that need to deal with overflow can implement
    /// this trait.
    ///
    /// The APIs with `_wrapping` suffix are the variant of non-overflow-checking. If overflow
    /// occurred, they will supposedly wrap around the boundary of the type.
    ///
    /// The APIs with `_checked` suffix are the variant of overflow-checking which return `None`
    /// if overflow occurred.
    pub trait ArrowNativeTypeOp:
        ArrowNativeType
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Zero
    {
        fn add_checked(self, rhs: Self) -> Result<Self> {
            Ok(self + rhs)
        }

        fn add_wrapping(self, rhs: Self) -> Self {
            self + rhs
        }

        fn sub_checked(self, rhs: Self) -> Result<Self> {
            Ok(self - rhs)
        }

        fn sub_wrapping(self, rhs: Self) -> Self {
            self - rhs
        }

        fn mul_checked(self, rhs: Self) -> Result<Self> {
            Ok(self * rhs)
        }

        fn mul_wrapping(self, rhs: Self) -> Self {
            self * rhs
        }

        fn div_checked(self, rhs: Self) -> Result<Self> {
            if rhs.is_zero() {
                Err(ArrowError::DivideByZero)
            } else {
                Ok(self / rhs)
            }
        }

        fn div_wrapping(self, rhs: Self) -> Self {
            self / rhs
        }
    }
}

macro_rules! native_type_op {
    ($t:tt) => {
        impl native_op::ArrowNativeTypeOp for $t {
            fn add_checked(self, rhs: Self) -> Result<Self> {
                self.checked_add(rhs).ok_or_else(|| {
                    ArrowError::ComputeError(format!(
                        "Overflow happened on: {:?} + {:?}",
                        self, rhs
                    ))
                })
            }

            fn add_wrapping(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }

            fn sub_checked(self, rhs: Self) -> Result<Self> {
                self.checked_sub(rhs).ok_or_else(|| {
                    ArrowError::ComputeError(format!(
                        "Overflow happened on: {:?} - {:?}",
                        self, rhs
                    ))
                })
            }

            fn sub_wrapping(self, rhs: Self) -> Self {
                self.wrapping_sub(rhs)
            }

            fn mul_checked(self, rhs: Self) -> Result<Self> {
                self.checked_mul(rhs).ok_or_else(|| {
                    ArrowError::ComputeError(format!(
                        "Overflow happened on: {:?} * {:?}",
                        self, rhs
                    ))
                })
            }

            fn mul_wrapping(self, rhs: Self) -> Self {
                self.wrapping_mul(rhs)
            }

            fn div_checked(self, rhs: Self) -> Result<Self> {
                if rhs.is_zero() {
                    Err(ArrowError::DivideByZero)
                } else {
                    self.checked_div(rhs).ok_or_else(|| {
                        ArrowError::ComputeError(format!(
                            "Overflow happened on: {:?} / {:?}",
                            self, rhs
                        ))
                    })
                }
            }

            fn div_wrapping(self, rhs: Self) -> Self {
                self.wrapping_div(rhs)
            }
        }
    };
}

native_type_op!(i8);
native_type_op!(i16);
native_type_op!(i32);
native_type_op!(i64);
native_type_op!(i128);
native_type_op!(u8);
native_type_op!(u16);
native_type_op!(u32);
native_type_op!(u64);

impl native_op::ArrowNativeTypeOp for f16 {}
impl native_op::ArrowNativeTypeOp for f32 {}
impl native_op::ArrowNativeTypeOp for f64 {}
