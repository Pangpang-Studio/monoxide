use std::{hash::Hash, sync::Arc};

/// A wrapper around [`Arc`] that uses the underlying pointer for comparison.
#[derive(Debug)]
pub struct RefIdArc<T>(Arc<T>);

impl<T> PartialEq for RefIdArc<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for RefIdArc<T> {}

impl<T> Hash for RefIdArc<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(Arc::as_ptr(&self.0) as usize)
    }
}

impl<T> From<Arc<T>> for RefIdArc<T> {
    fn from(value: Arc<T>) -> Self {
        RefIdArc(value)
    }
}

impl<T> From<RefIdArc<T>> for Arc<T> {
    fn from(value: RefIdArc<T>) -> Self {
        value.0
    }
}
