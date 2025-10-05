use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::{cmp, fmt, hash};

#[derive(Clone)]
pub struct Arena<T>(slotmap::SlotMap<Idx<T>, T>);

impl<T> Arena<T> {
    pub fn insert(&mut self, value: T) -> Idx<T> {
        self.0.insert(value)
    }

    pub fn remove(&mut self, id: Idx<T>) -> Option<T> {
        self.0.remove(id)
    }

    pub fn get(&self, id: Idx<T>) -> Option<&T> {
        self.0.get(id)
    }

    pub fn get_mut(&mut self, id: Idx<T>) -> Option<&mut T> {
        self.0.get_mut(id)
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn iter(&self) -> slotmap::basic::Iter<Idx<T>, T> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> slotmap::basic::IterMut<Idx<T>, T> {
        self.0.iter_mut()
    }

    pub fn values(&self) -> slotmap::basic::Values<Idx<T>, T> {
        self.0.values()
    }

    pub fn values_mut(&mut self) -> slotmap::basic::ValuesMut<Idx<T>, T> {
        self.0.values_mut()
    }
}

impl<T> Index<Idx<T>> for Arena<T> {
    type Output = T;

    fn index(&self, index: Idx<T>) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<Idx<T>> for Arena<T> {
    fn index_mut(&mut self, index: Idx<T>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: fmt::Debug> fmt::Debug for Arena<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Arena").field("len", &self.0.len()).field("data", &self.0).finish()
    }
}

pub struct Idx<T> {
    data: slotmap::KeyData,
    _phantom: PhantomData<fn(T)>,
}

impl<T> Default for Arena<T> {
    fn default() -> Arena<T> {
        Arena(Default::default())
    }
}

unsafe impl<T> slotmap::Key for Idx<T> {
    fn data(&self) -> slotmap::KeyData {
        self.data
    }
}

impl<T> From<slotmap::KeyData> for Idx<T> {
    fn from(data: slotmap::KeyData) -> Self {
        Idx { data, _phantom: PhantomData }
    }
}

impl<T> Clone for Idx<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Idx<T> {}

impl<T> Default for Idx<T> {
    fn default() -> Self {
        <Idx<T>>::from(slotmap::KeyData::default())
    }
}

impl<T> PartialOrd for Idx<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<T> Ord for Idx<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        Ord::cmp(&self.data, &other.data)
    }
}

impl<T> PartialEq for Idx<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T> Eq for Idx<T> {}

impl<T> fmt::Debug for Idx<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Idx").field(&self.data).finish()
    }
}

impl<T> hash::Hash for Idx<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state)
    }
}
