use std::hash::Hash;

/// A borrowed pointer which uses the underlying pointer for equality.
#[derive(Debug)]
pub struct RefId<'a, T>(&'a T);

impl<'a, T> RefId<'a, T> {
    /// Create a new `RefId` from a reference.
    pub fn new(reference: impl Into<&'a T>) -> Self {
        RefId(reference.into())
    }

    /// Get the underlying reference.
    pub fn as_ref(&self) -> &'a T {
        self.0
    }
}

impl<'a, T> From<&'a T> for RefId<'a, T> {
    fn from(reference: &'a T) -> Self {
        RefId(reference)
    }
}

impl<'a, T> PartialEq for RefId<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl<'a, T> Eq for RefId<'a, T> {}

impl<'a, T> Hash for RefId<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let ptr = self.0 as *const T;
        ptr.hash(state);
    }
}
