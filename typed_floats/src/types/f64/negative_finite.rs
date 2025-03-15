use crate::types::{f64, InvalidNumber, NegativeFinite};
use const_fn::const_fn;

impl NegativeFinite<f64> {
    /// Creates a new value from a primitive type
    /// It adds a little overhead compared to `new_unchecked`
    /// because it checks that the value is valid
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::tf64::NegativeFinite;
    /// let x = NegativeFinite::new(-3.0).unwrap();
    ///
    /// assert_eq!(x, -3.0);
    /// ```
    ///
    /// # Errors
    /// Returns an error if the value is not valid
    #[inline]
    #[const_fn("1.83")]
    pub fn new(value: f64) -> Result<Self, InvalidNumber> {
        if value.is_nan() {
            return Err(InvalidNumber::NaN);
        }

        if value.is_infinite() {
            return Err(InvalidNumber::Infinite);
        }

        if value.is_sign_positive() {
            return Err(InvalidNumber::Positive);
        }

        Ok(Self(value))
    }

    /// Creates a new value from a primitive type with zero overhead (in release mode).
    /// It is up to the caller to ensure that the value is valid
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::tf64::NegativeFinite;
    /// let x = unsafe { NegativeFinite::new_unchecked(-3.0) };
    ///
    /// assert_eq!(x, -3.0);
    /// ```
    /// # Safety
    /// The caller must ensure that the value is valid.
    /// It will panic in debug mode if the value is not valid,
    /// but in release mode the behavior is undefined
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub const unsafe fn new_unchecked(value: f64) -> Self {
        if Self::new(value).is_err() || value > 0.0 {
            #[cfg(debug_assertions)]
            panic!("This value is not a valid NegativeFinite<f64>");

            #[cfg(feature = "ensure_no_undefined_behavior")]
            panic!("This value is not a valid NegativeFinite<f64>");

            #[cfg(all(
                feature = "compiler_hints",
                not(feature = "ensure_no_undefined_behavior")
            ))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        }

        Self(value)
    }

    /// Returns the value as a primitive type
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NegativeFinite;
    ///
    /// let x = NegativeFinite::new(-3.0).unwrap();
    ///
    /// let y: f64 = x.into();
    ///
    /// assert_eq!(y, -3.0);
    /// ```
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn get(&self) -> f64 {
        self.0
    }

    /// Returns `true` if this value is NaN.
    /// This is never the case for the provided types
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NegativeFinite;
    /// let x: NegativeFinite = (-3.0).try_into().unwrap();
    ///
    /// assert_eq!(x.is_nan(), false);
    /// ```
    ///
    /// See [`f64::is_nan()`] for more details.
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
    /// use typed_floats::tf64::NegativeFinite;
    /// let x: NegativeFinite = (-3.0).try_into().unwrap();
    ///
    /// assert_eq!(x.is_infinite(), false);
    /// ```
    ///
    /// See [`f64::is_infinite()`] for more details.
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn is_infinite(&self) -> bool {
        false
    }

    /// Returns `true` if this number is positive infinity nor negative infinity.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NegativeFinite;
    /// let x: NegativeFinite = (-3.0).try_into().unwrap();
    ///
    /// assert_eq!(x.is_finite(), true);
    /// ```
    ///
    /// See [`f64::is_finite()`] for more details.
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn is_finite(&self) -> bool {
        true
    }

    /// Returns `true` if the number is [subnormal](https://en.wikipedia.org/wiki/Denormal_number).
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NegativeFinite;
    /// let x: NegativeFinite = (-3.0).try_into().unwrap();
    ///
    /// assert_eq!(x.is_subnormal(), false);
    /// ```
    ///
    /// See [`f64::is_subnormal()`] for more details.
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
    /// use typed_floats::tf64::NegativeFinite;
    /// let x: NegativeFinite = (-3.0).try_into().unwrap();
    ///
    /// assert_eq!(x.is_normal(), true);
    /// ```
    ///
    /// See [`f64::is_normal()`] for more details.
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
    /// use typed_floats::tf64::NegativeFinite;
    /// let x: NegativeFinite = (-3.0).try_into().unwrap();
    ///
    /// assert_eq!(x.classify(), core::num::FpCategory::Normal);
    /// ```
    ///
    /// See [`f64::classify()`] for more details.
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
    /// use typed_floats::tf64::NegativeFinite;
    /// let x: NegativeFinite = (-3.0).try_into().unwrap();
    ///
    /// assert_eq!(x.is_sign_positive(), false);
    /// ```
    ///
    /// See [`f64::is_sign_positive()`] for more details.
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn is_sign_positive(&self) -> bool {
        false
    }

    /// Returns `true` if `self` has a negative sign, including `-0.0` and negative infinity.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NegativeFinite;
    /// let x: NegativeFinite = (-3.0).try_into().unwrap();
    ///
    /// assert_eq!(x.is_sign_negative(), true);
    /// ```
    ///
    /// See [`f64::is_sign_negative()`] for more details.
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn is_sign_negative(&self) -> bool {
        true
    }

    /// Returns `true` if the number is negative zero.
    ///     
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NegativeFinite;
    /// let x: NegativeFinite = (-3.0).try_into().unwrap();
    /// let y: NegativeFinite = (-0.0).try_into().unwrap();
    ///
    /// assert_eq!(x.is_negative_zero(), false);
    /// assert_eq!(y.is_negative_zero(), true);
    /// ```
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn is_negative_zero(&self) -> bool {
        self.0 == 0.0
    }

    /// Returns `true` if the number is positive zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NegativeFinite;
    /// let x: NegativeFinite = (-3.0).try_into().unwrap();
    /// let y: NegativeFinite = (-0.0).try_into().unwrap();
    ///
    /// assert_eq!(x.is_positive_zero(), false);
    /// assert_eq!(y.is_positive_zero(), false);
    /// ```
    #[inline]
    #[must_use]
    #[const_fn("1.83")]
    pub fn is_positive_zero(&self) -> bool {
        false
    }
}
