type TFCoord = crate::NonNaNFinite<f64>;
pub struct TFLatlng(TFCoord, TFCoord);

impl Into<h3o::LatLng> for TFLatlng {
    fn into(self) -> h3o::LatLng {
        // SAFETY: `NonNaNFinite` guarantees that the values are finite.
        match h3o::LatLng::new(self.0.into(), self.1.into()) {
            Ok(latlng) => latlng,
            Err(_) => unreachable!(),
        }
    }
}
