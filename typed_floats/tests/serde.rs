use typed_floats::*;


#[cfg(feature = "serde")]
#[test]
fn test_serde() {
    let a: Positive<f64> = 3.0f64.try_into().unwrap();
    
    let serialized = serde_json::to_string(&a).unwrap();

    assert_eq!(serialized, "3.0");
}