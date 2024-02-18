#![allow(clippy::comparison_chain)]

use crate::{Negative, NegativeFinite, NonNaN, NonNaNFinite, Positive, PositiveFinite};

macro_rules! impl_default {
    ($test:ident, $type:ident, $default:expr) => {
        impl core::default::Default for $type<f32> {
            fn default() -> Self {
                // # Safety
                // This is safe because the value is valid for that type.
                unsafe { Self::new_unchecked($default) }
            }
        }

        impl core::default::Default for $type<f64> {
            fn default() -> Self {
                // # Safety
                // This is safe because the value is valid for that type.
                unsafe { Self::new_unchecked($default) }
            }
        }

        #[cfg(test)]
        mod $test {
            use crate::{Negative, NegativeFinite, NonNaN, NonNaNFinite, Positive, PositiveFinite};

            #[derive(Default)]
            struct SomeOptions {
                foo: f32,
                bar: f64,
                baz: $type<f32>,
                qux: $type<f64>,
            }

            #[test]
            fn test() {
                let options: SomeOptions = Default::default();

                // N.B. The assert succeeds if the value is `0.0` or `-0.0
                assert_eq!(options.foo, 0.0);
                assert_eq!(options.bar, 0.0);
                assert_eq!(options.baz.get(), 0.0);
                assert_eq!(options.qux.get(), 0.0);
            }
        }
    };
}

impl_default!(non_nan, NonNaN, 0.0);
impl_default!(non_nan_finite, NonNaNFinite, 0.0);
impl_default!(positive, Positive, 0.0);
impl_default!(negative, Negative, -0.0);
impl_default!(positive_finite, PositiveFinite, 0.0);
impl_default!(negative_finite, NegativeFinite, -0.0);
