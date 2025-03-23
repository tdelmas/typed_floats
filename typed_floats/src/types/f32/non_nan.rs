use crate::types::{f32, InvalidNumber, NonNaN};
use const_fn::const_fn;

impl NonNaN<f32> {
    /// Creates a new value from a primitive type
    /// It adds a little overhead compared to `new_unchecked`
    /// because it checks that the value is valid
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::tf32::NonNaN;
    /// let x = NonNaN::new(3.0).unwrap();
    ///
    /// assert_eq!(x, 3.0);
    /// ```
    ///
    /// # Errors
    /// Returns an error if the value is not valid
    #[inline]
    #[const_fn("1.83")]
    pub fn new(value: f32) -> Result<Self, InvalidNumber> {
        if value.is_nan() {
            return Err(InvalidNumber::NaN);
        }

        Ok(Self(value))
    }

    /// Creates a new value from a primitive type with zero overhead (in release mode).
    /// It is up to the caller to ensure that the value is valid
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::tf32::NonNaN;
    /// let x = unsafe { NonNaN::new_unchecked(3.0) };
    ///
    /// assert_eq!(x, 3.0);
    /// ```
    /// # Safety
    /// The caller must ensure that the value is valid.
    /// It will panic in debug mode if the value is not valid,
    /// but in release mode the behavior is undefined
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub const unsafe fn new_unchecked(value: f32) -> Self {
        crate::macros::new_unchecked!(value, NonNaN)
    }

    /// Returns the value as a primitive type
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf32::NonNaN;
    ///
    /// let x = NonNaN::new(3.0).unwrap();
    ///
    /// let y: f32 = x.into();
    ///
    /// assert_eq!(y, 3.0);
    /// ```
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn get(&self) -> f32 {
        self.0
    }

    /// Returns `true` if this value is NaN.
    /// This is never the case for the provided types
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf32::NonNaN;
    /// let x: NonNaN = 3.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_nan(), false);
    /// ```
    ///
    /// See [`f32::is_nan()`] for more details.
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn is_nan(&self) -> bool {
        false
    }

    /// Returns `true` if this value is positive infinity or negative infinity.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf32::NonNaN;
    /// let x: NonNaN = 3.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_infinite(), false);
    /// ```
    ///
    /// See [`f32::is_infinite()`] for more details.
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn is_infinite(&self) -> bool {
        self.0.is_infinite()
    }

    /// Returns `true` if this number is positive infinity nor negative infinity.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf32::NonNaN;
    /// let x: NonNaN = 3.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_finite(), true);
    /// ```
    ///
    /// See [`f32::is_finite()`] for more details.
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    /// Returns `true` if the number is [subnormal](https://en.wikipedia.org/wiki/Denormal_number).
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf32::NonNaN;
    /// let x: NonNaN = 3.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_subnormal(), false);
    /// ```
    ///
    /// See [`f32::is_subnormal()`] for more details.
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn is_subnormal(&self) -> bool {
        self.0.is_subnormal()
    }

    /// Returns `true` if the number is neither zero, infinite or [subnormal](https://en.wikipedia.org/wiki/Denormal_number).
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf32::NonNaN;
    /// let x: NonNaN = 3.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_normal(), true);
    /// ```
    ///
    /// See [`f32::is_normal()`] for more details.
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn is_normal(&self) -> bool {
        self.0.is_normal()
    }

    /// Returns the floating point category of the number. If only one property
    /// is going to be tested, it is generally faster to use the specific
    /// predicate instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf32::NonNaN;
    /// let x: NonNaN = 3.0.try_into().unwrap();
    ///
    /// assert_eq!(x.classify(), core::num::FpCategory::Normal);
    /// ```
    ///
    /// See [`f32::classify()`] for more details.
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn classify(&self) -> core::num::FpCategory {
        self.0.classify()
    }

    /// Returns `true` if `self` has a positive sign, including `+0.0` and positive infinity.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf32::NonNaN;
    /// let x: NonNaN = 3.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_sign_positive(), true);
    /// ```
    ///
    /// See [`f32::is_sign_positive()`] for more details.
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn is_sign_positive(&self) -> bool {
        self.0.is_sign_positive()
    }

    /// Returns `true` if `self` has a negative sign, including `-0.0` and negative infinity.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf32::NonNaN;
    /// let x: NonNaN = 3.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_sign_negative(), false);
    /// ```
    ///
    /// See [`f32::is_sign_negative()`] for more details.
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn is_sign_negative(&self) -> bool {
        self.0.is_sign_negative()
    }

    /// Returns `true` if the number is negative zero.
    ///     
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf32::NonNaN;
    /// let x: NonNaN = 3.0.try_into().unwrap();
    /// let y: NonNaN = (-0.0).try_into().unwrap();
    /// let z: NonNaN = 0.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_negative_zero(), false);
    /// assert_eq!(y.is_negative_zero(), true);
    /// assert_eq!(z.is_negative_zero(), false);
    /// ```
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn is_negative_zero(&self) -> bool {
        self.0 == 0.0 && self.0.is_sign_negative()
    }

    /// Returns `true` if the number is positive zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf32::NonNaN;
    /// let x: NonNaN = 3.0.try_into().unwrap();
    /// let y: NonNaN = (-0.0).try_into().unwrap();
    /// let z: NonNaN = 0.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_positive_zero(), false);
    /// assert_eq!(y.is_positive_zero(), false);
    /// assert_eq!(z.is_positive_zero(), true);
    /// ```
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn is_positive_zero(&self) -> bool {
        self.0 == 0.0 && self.0.is_sign_positive()
    }
}
