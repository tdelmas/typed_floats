// mod layout_optimisation;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "serde")]
pub trait Contained: Serialize {}
#[cfg(not(feature = "serde"))]
pub trait Contained {}

impl Contained for f32 {}
impl Contained for f64 {}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub(crate) struct Container<T: Contained> (T);

impl<T: Contained> Container<T> {
    pub const fn new(value: T) -> Container<T> {
        Self(value)
    }

    pub const fn get(&self) -> &T {
        &self.0
    }
}
