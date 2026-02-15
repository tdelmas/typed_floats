#![cfg(feature = "serde")]

#[cfg(feature = "f64")]
mod test_f64 {
    use typed_floats::*;

    #[test]
    fn test_serde_serialize() {
        let a: Positive<f64> = 3.0f64.try_into().unwrap();

        let serialized = serde_json::to_string(&a).unwrap();

        assert_eq!(serialized, "3.0");
    }

    #[test]
    fn test_serde_deserialize() {
        let json = "3.0";
        let a: Positive<f64> = serde_json::from_str(json).unwrap();

        assert_eq!(a, 3.0f64);

        let json = "-3.0";
        let a: Result<Positive<f64>, _> = serde_json::from_str(json);

        assert!(a.is_err());
        assert_eq!(a.unwrap_err().to_string(), "Number is negative");
    }

    #[test]
    fn test_serde_struct() {
        use serde::Serialize;

        let map = serde_json::json!({
            "a": 1.0,
        });

        #[derive(Serialize)]
        struct A {
            a: NonNaN,
        }

        let a = A {
            a: NonNaN::try_from(1.0).unwrap(),
        };

        let a_json = serde_json::to_value(a).unwrap();

        assert_eq!(a_json, map);
    }
}

#[cfg(feature = "f32")]
mod test_f32 {
    use typed_floats::*;

    #[test]
    fn test_serde_serialize() {
        let a: Positive<f32> = 3.0f32.try_into().unwrap();

        let serialized = serde_json::to_string(&a).unwrap();

        assert_eq!(serialized, "3.0");
    }

    #[test]
    fn test_serde_deserialize() {
        let json = "3.0";
        let a: Positive<f32> = serde_json::from_str(json).unwrap();

        assert_eq!(a, 3.0f32);

        let json = "-3.0";
        let a: Result<Positive<f32>, _> = serde_json::from_str(json);

        assert!(a.is_err());
        assert_eq!(a.unwrap_err().to_string(), "Number is negative");
    }

    #[test]
    fn test_serde_struct() {
        use serde::Serialize;

        let map = serde_json::json!({
            "a": 1.0,
        });

        #[derive(Serialize)]
        struct A {
            a: NonNaN<f32>,
        }

        let a = A {
            a: NonNaN::try_from(1.0f32).unwrap(),
        };

        let a_json = serde_json::to_value(a).unwrap();

        assert_eq!(a_json, map);
    }
}
