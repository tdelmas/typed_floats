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

    #[test]
    fn test_option_size_32() {
        const SIZE_F32: usize = core::mem::size_of::<f32>();
        const SIZE_OPTION_F32: usize = core::mem::size_of::<Option<f32>>();
        
        const SIZE_NONZERO_TF32: usize = core::mem::size_of::<NonZeroF32Packed>();
        const SIZE_OPTION_NONZERO_TF32: usize = core::mem::size_of::<Option<NonZeroF32Packed>>();

        assert_eq!(SIZE_F32, SIZE_NONZERO_TF32);
        assert_eq!(SIZE_NONZERO_TF32, SIZE_OPTION_NONZERO_TF32);


        assert!(SIZE_OPTION_F32 > SIZE_F32);
    }

    #[test]
    fn test_option_size_64() {
        const SIZE_F64: usize = core::mem::size_of::<f64>();
        const SIZE_OPTION_F64: usize = core::mem::size_of::<Option<f64>>();
        
        const SIZE_NONZERO_TF64: usize = core::mem::size_of::<NonZeroF64Packed>();
        const SIZE_OPTION_NONZERO_TF64: usize = core::mem::size_of::<Option<NonZeroF64Packed>>();

        assert_eq!(SIZE_F64, SIZE_NONZERO_TF64);
        assert_eq!(SIZE_NONZERO_TF64, SIZE_OPTION_NONZERO_TF64);


        assert!(SIZE_OPTION_F64 > SIZE_F64);
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
