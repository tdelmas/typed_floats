use typed_floats::*;

#[test]
fn test_exclude_ok() {
    let nonnan: NonNaN<f32> = 1.0.try_into().unwrap();
    let positive: Positive<f32> = nonnan.exclude_negative().unwrap();
    let finite: PositiveFinite<f32> = positive.exclude_inf().unwrap();
    let nonzero: Result<StrictlyPositiveFinite<f32>, PositiveFinite<f32>> = finite.exclude_zero();
    assert!(nonzero.is_ok());
}

#[test]
fn test_exclude_err() {
    let nonnan: NonNaN<f32> = 0.0.try_into().unwrap();
    let positive: Positive<f32> = nonnan.exclude_negative().unwrap();
    let finite: PositiveFinite<f32> = positive.exclude_inf().unwrap();
    let err: Result<StrictlyPositiveFinite<f32>, PositiveFinite<f32>> = finite.exclude_zero();
    assert_eq!(err, Err(finite));
}
