use std::ops::Deref;
use std::ptr;

use std::sync::Arc;
#[derive(Debug,Hash)]
pub struct Cell<T: ?Sized>(Arc<T>);

impl<T:?Sized> Cell<T> {
    pub fn new(value: Arc<T>) -> Self {
        Self(value)
    }
}
#[cfg(test)]
mod cell_test {
    use std::sync::Arc;

    use super::Cell;
    #[test]
    fn test() {
        let a = Cell::new(Arc::new(5));
        let b = Cell::new(Arc::new(5));
        assert!(a != b);
    }
}

impl<T: ?Sized> Clone for Cell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized> PartialEq for Cell<T> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(Arc::as_ptr(&self.0), Arc::as_ptr(&other.0))
    }
}
impl<T: ?Sized> Eq for Cell<T> {}


impl<T:? Sized> Deref for Cell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}