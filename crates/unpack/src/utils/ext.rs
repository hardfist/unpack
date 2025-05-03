use std::any::Any;
use std::hash::Hash;
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait IntoAny {
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any>;
}
impl<T: Any> IntoAny for T {
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

pub trait DynHash {
    fn dyn_hash(&self, state: &mut dyn std::hash::Hasher);
}

impl<T: Hash> DynHash for T {
    fn dyn_hash(&self, mut state: &mut dyn std::hash::Hasher) {
        self.hash(&mut state);
    }
}
