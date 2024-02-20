type TFCoord = crate::NonNaNFinite<f64>;

/// Intermediate type because "tuples are always foreign"
pub struct TFLatlng(pub TFCoord, pub TFCoord);

impl From<TFLatlng> for h3o::LatLng {
    fn from(val: TFLatlng) -> Self {
        // SAFETY: `NonNaNFinite` guarantees that the values are finite.
        Self::new(val.0.into(), val.1.into()).map_or_else(|_| unreachable!(), |latlng| latlng)
    }
}
