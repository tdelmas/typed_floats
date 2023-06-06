//! zero overhead :everything is checked at compile time.
//! (only `try_from` adds a little overhead at runtime)
//!
//! Better to have too much .`into`() than .`try_into`() for the conversion.

//! Warning:
//! The test == 0.0f64 is true for +0.0f64 and -0.0f64.
//! To distinguish between +0.0f64 and -0.0f64,
//! the code use the method `is_sign_positive`().

//! Sanity:
//! When an operation car result in a NaN, the operation is not implemented.
//! To use it, you have to .into<f64>() the value first.
//!
//! Infinity times zero is NaN.

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::panic)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::indexing_slicing)]

mod ops;
mod types;

pub use types::*;
#[cfg(test)]
mod tests {

    use crate::types::*;

    const ZERO: f64 = 0.0f64;
    const NEGATIVE_ZERO: f64 = -0.0f64;

    macro_rules! test_one {
        ($values:ident,$T:ty) => {
            for a in $values.iter() {
                let a = <$T>::try_from(*a);

                match a {
                    Ok(num_a) => {
                        println!("num={}", num_a);
                        println!("neg={}", -num_a);
                        println!("abs={}", num_a.abs());
                    }
                    Err(_) => {}
                }
            }
        };
    }

    macro_rules! assert_eq_nan {
        ($a:expr, $b:expr, $($arg:tt)+ ) => {
            let a_is_nan = $a != $a;
            let b_is_nan = $b != $b;

            assert_eq!(a_is_nan, b_is_nan, $($arg)+);

            if (!a_is_nan && !b_is_nan) {
                assert_eq!($a, $b, $($arg)+);
            }
        };
    }

    macro_rules! assert_is_smalest_container {
        ($vec:expr) => {
            //let smallest_container =
            //if found smaller container that can handle all the values, ok.
            0;
        };
    }

    macro_rules! test_all {
        ($values:ident,$T:ty,$U:ty) => {
            let mut adds = Vec::new();

            for a in $values.iter() {
                let a = <$T>::try_from(*a);

                match a {
                    Ok(num_a) => {
                        for b in $values.iter() {
                            let b = <$U>::try_from(*b);

                            match b {
                                Ok(num_b) => {
                                    println!("num_a={}", num_a);
                                    println!("num_b={}", num_b);

                                    let add = num_a + num_b;
                                    adds.push(add);

                                    println!("add={}", add);
                                    println!("mul={}", num_a * num_b);
                                    println!("div={}", num_a / num_b);
                                    println!("mod={}", num_a % num_b);

                                    assert_eq_nan!(
                                        num_a + num_b,
                                        num_b + num_a,
                                        "{} + {} != {} + {}",
                                        num_a,
                                        num_b,
                                        num_b,
                                        num_a
                                    );
                                    assert_eq_nan!(
                                        num_a * num_b,
                                        num_b * num_a,
                                        "{} * {} != {} * {}",
                                        num_a,
                                        num_b,
                                        num_b,
                                        num_a
                                    );
                                }
                                Err(_) => {}
                            }
                        }
                    }
                    Err(_) => {}
                }
            }

            assert_is_smalest_container!(adds)
        };
    }

    macro_rules! impl_for_one {
        ($values:ident,$($name:ty,)*) => {
            $(
                test_one!($values, $name);
            )*
        };
    }

    macro_rules! impl_for_all {
        ($values:ident,$($name:ty,)*) => {
            $(
                test_all!($values, $name, NonNaN);
            )*
        };
    }

    #[test]
    fn test_all() {
        let values = [
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::MAX,
            f64::MIN,
            f64::MIN_POSITIVE,
            -f64::MIN_POSITIVE,
            ZERO,
            NEGATIVE_ZERO,
            1.0,
            -1.0,
        ];

        impl_for_one!(
            values,
            NonNaN,
            NonZeroNonNaN,
            Positive,
            Negative,
            StrictlyPositive,
            StrictlyNegative,
            StrictlyNegativeFinite,
            StrictlyPositiveFinite,
            NegativeFinite,
            PositiveFinite,
        );

        impl_for_all!(
            values,
            NonNaN,
            NonZeroNonNaN,
            Positive,
            Negative,
            StrictlyPositive,
            StrictlyNegative,
            StrictlyNegativeFinite,
            StrictlyPositiveFinite,
            NegativeFinite,
            PositiveFinite,
        );
    }

    #[test]
    fn test_others() {
        let values = [
            (f64::NAN, false),
            (f64::INFINITY, true),
            (f64::NEG_INFINITY, true),
            (ZERO, true),
            (NEGATIVE_ZERO, true),
            (1.0, true),
            (-1.0, true),
        ];

        for (value, expected) in values.iter() {
            let num = NonNaN::try_from(*value);
            let result = num.is_ok();
            assert_eq!(result, *expected);

            match num {
                Ok(num) => {
                    let v: f64 = num.into();
                    assert_eq!(v, *value);
                }
                Err(_) => {}
            }
        }

        let a: NonNaN = 1.0.try_into().unwrap();
        let b: Positive = a.into();

        assert_eq!(1.0, b.into());
    }
}
