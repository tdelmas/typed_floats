use core::num::{NonZeroU32, NonZeroU64};
use core::ops::Not;

pub trait Packed<T> {
    fn set(value: T) -> Self;
    fn get(&self) -> T;
}

pub struct NonZeroF32Packed(NonZeroU32);
pub struct NonZeroF64Packed(NonZeroU64);
pub struct NonNaNF32Packed(NonZeroU32);
pub struct NonNaNF64Packed(NonZeroU64);

impl Packed<f32> for NonZeroF32Packed {
    fn set(value: f32) -> Self {
        let bits = value.to_bits();
        debug_assert!(bits != 0, "Value cannot be zero");
        unsafe { NonZeroF32Packed(NonZeroU32::new_unchecked(bits)) }
    }

    fn get(&self) -> f32 {
        f32::from_bits(self.0.get())
    }
}

impl Packed<f64> for NonZeroF64Packed {
    fn set(value: f64) -> Self {
        let bits = value.to_bits();
        debug_assert!(bits != 0, "Value cannot be zero");
        unsafe { NonZeroF64Packed(NonZeroU64::new_unchecked(bits)) }
    }

    fn get(&self) -> f64 {
        f64::from_bits(self.0.get())
    }
}

impl Packed<f32> for NonNaNF32Packed {
    fn set(value: f32) -> Self {
        let bits = value.to_bits().not();
        debug_assert!(bits != 0, "Value cannot be NaN");
        unsafe { NonNaNF32Packed(NonZeroU32::new_unchecked(bits)) }
    }

    fn get(&self) -> f32 {
        f32::from_bits(self.0.get().not())
    }
}

impl Packed<f64> for NonNaNF64Packed {
    fn set(value: f64) -> Self {
        let bits = value.to_bits().not();
        debug_assert!(bits != 0, "Value cannot be NaN");
        unsafe { NonNaNF64Packed(NonZeroU64::new_unchecked(bits)) }
    }

    fn get(&self) -> f64 {
        f64::from_bits(self.0.get().not())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALUES_F32: [f32; 5] = [1.0, -1.0, 3.14, -2.71, 42.0];
    const VALUES_F64: [f64; 5] = [1.0, -1.0, 3.14, -2.71, 42.0];

    type OptionNonZeroF32 = Option<NonZeroF32Packed>;
    type OptionNonZeroF64 = Option<NonZeroF64Packed>;
    type OptionNonNaNF32 = Option<NonNaNF32Packed>;
    type OptionNonNaNF64 = Option<NonNaNF64Packed>;

    #[test]
    fn test_option_size() {
        assert!(core::mem::size_of::<Option<f32>>() > core::mem::size_of::<f32>());
        assert!(core::mem::size_of::<Option<f64>>() > core::mem::size_of::<f64>());

        assert_eq!(
            core::mem::size_of::<OptionNonZeroF32>(),
            core::mem::size_of::<f32>()
        );
        assert_eq!(
            core::mem::size_of::<OptionNonZeroF64>(),
            core::mem::size_of::<f64>()
        );
        assert_eq!(
            core::mem::size_of::<OptionNonNaNF32>(),
            core::mem::size_of::<f32>()
        );
        assert_eq!(
            core::mem::size_of::<OptionNonNaNF64>(),
            core::mem::size_of::<f64>()
        );
    }

    #[test]
    fn test_non_zero_f32() {
        for &value in &VALUES_F32 {
            let packed = NonZeroF32Packed::set(value);
            let unpacked = packed.get();
            assert_eq!(value, unpacked);
        }
    }

    #[test]
    fn test_non_zero_f64() {
        for &value in &VALUES_F64 {
            let packed = NonZeroF64Packed::set(value);
            let unpacked = packed.get();
            assert_eq!(value, unpacked);
        }
    }

    #[test]
    fn test_non_nan_f32() {
        for &value in &VALUES_F32 {
            let packed = NonNaNF32Packed::set(value);
            let unpacked = packed.get();
            assert_eq!(value, unpacked);
        }

        let zero = 0.0f32;
        let packed_zero = NonNaNF32Packed::set(zero);
        let unpacked_zero = packed_zero.get();
        assert_eq!(zero, unpacked_zero);
    }

    #[test]
    fn test_non_nan_f64() {
        for &value in &VALUES_F64 {
            let packed = NonNaNF64Packed::set(value);
            let unpacked = packed.get();
            assert_eq!(value, unpacked);
        }

        let zero = 0.0f64;
        let packed_zero = NonNaNF64Packed::set(zero);
        let unpacked_zero = packed_zero.get();
        assert_eq!(zero, unpacked_zero);
    }
}
