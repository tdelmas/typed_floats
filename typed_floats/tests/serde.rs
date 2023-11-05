#![cfg(feature = "serde")]

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
