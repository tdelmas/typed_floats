#![cfg(any(feature = "std", feature = "libm"))]

use typed_floats::*;

typed_floats_macros::generate_tests_self!(atanh);
