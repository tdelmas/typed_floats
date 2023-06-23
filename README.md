This crate helps you to ensure the kind of floats you are using.

# NOT PRODUCTION READY YET

zero overhead :everything is checked at compile time.
(only `try_from` adds a little overhead at runtime)

`NaN` is rejected by all types.

The types provided by this crate are:
- `NonNaN`,`NonNaNFinite`, `NonZeroNonNaN`, `NonZeroNonNaNFinite`
Their positive and negative counterparts:
- `Positive`,`PositiveFinite`, `StrictlyPositive`, `StrictlyPositiveFinite`
- `Negative`,`NegativeFinite`, `StrictlyNegative`, `StrictlyNegativeFinite`

By default all types are `f64` but you can use the `f32` like `Positive<f32>`.

The following conversions are implemented:
- Between all the types of this crate
- From `f64`
- From integers types (expect `u128` and `i128`)
- From `NonZero*`


#FIXME: 
    println!("Hello,{} =zero? {}", f64::MIN_POSITIVE / f64::MAX, 0.0 == (f64::MIN_POSITIVE / f64::MAX));