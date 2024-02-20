#[cfg(feature = "into-h3o")]
#[test]
fn latlng_from_typed_floats() {
    let lat = typed_floats::NonNaNFinite::<f64>::new(48.864716).unwrap();
    let lng = typed_floats::NonNaNFinite::<f64>::new(2.349014).unwrap();

    // Intermediate type because "tuples are always foreign"
    let latlng = typed_floats::h3o::TFLatlng(lat, lng);

    let _: h3o::LatLng = latlng.into();
}
