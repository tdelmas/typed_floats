#[cfg(test)]
mod tests {
    #[test]
    fn with_checked_float() {
        use checked_float::*;

        #[derive(Debug)]
        struct NanError;

        struct NoNanChecker;
        impl<T: Float> FloatChecker<T> for NoNanChecker {
            type Error = NanError;
            fn check(value: T) -> Result<(), Self::Error> {
                if value.is_nan() {
                    Err(NanError)
                } else {
                    Ok(())
                }
            }
        }

        type NoNan64 = CheckedFloat<f64, NoNanChecker>; // our checked float wrapper

        let x = NoNan64::new(2.0).unwrap(); // not nan, so we can unwrap
        let y = NoNan64::new(0.0).unwrap(); // not nan, so we can unwrap

        assert!(y.div(y).is_err()); // 0/0 is nan, so we get Err
    }

    #[test]
    fn with_typed_floats() {
        use typed_floats::tf64::NonNaN;

        let x = NonNaN::new(2.0).unwrap(); // not nan, so we can unwrap
        let y = NonNaN::new(0.0).unwrap(); // not nan, so we can unwrap

        //TODO assert_eq!(x.powf(y).try_into::<NonNaN<f64>>().unwrap().get(), 1.0); // not nan, so we can unwrap

        let z: Result<NonNaN, _> = (y / y).try_into(); // 0/0 is nan, so we get Err
        assert!(z.is_err());
    }
}

fn main() {}
