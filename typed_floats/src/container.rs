// mod layout_optimisation;

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub(crate) struct Container<T> (T);

impl<T> Container<T> {
    pub const fn new(value: T) -> Container<T> {
        Self(value)
    }

    pub const fn get(&self) -> &T {
        &self.0
    }
}
