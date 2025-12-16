use dashmap::{DashMap, SharedValue};
use std::hash::{BuildHasher, Hash, Hasher};
use std::ops::Deref;
use std::{cmp, fmt};
use triomphe::Arc;

#[derive(Debug)]
pub struct InternMap<T: Hash + Eq + 'static> {
    storage: Storage<T>,
}

type Storage<T> = Arc<DashMap<Arc<T>, ()>>;

pub struct Interned<T: Hash + Eq + 'static> {
    storage: Storage<T>,
    value: Arc<T>,
}

impl<T: Hash + Eq + 'static> InternMap<T> {
    pub fn alloc(&self, obj: T) -> Interned<T> {
        let hash = self.storage.hasher().hash_one(&obj);

        let index = self.storage.determine_shard(hash as usize);
        let shard = &self.storage.shards()[index];
        let mut shard = shard.write();

        // Safety: shard locked
        let obj = unsafe {
            match shard.find_or_find_insert_slot(
                hash,
                |v| obj == *v.0,
                |v| self.storage.hasher().hash_one(&*v.0),
            ) {
                Ok(bucket) => Arc::clone(&bucket.as_ref().0),
                Err(slot) => {
                    let obj = Arc::new(obj);
                    let bucket = shard.insert_in_slot(hash, slot, (obj, SharedValue::new(())));
                    Arc::clone(&bucket.as_ref().0)
                }
            }
        };

        Interned {
            storage: self.storage.clone(),
            value: obj,
        }
    }
}

impl<T: Hash + Eq + 'static> Interned<T> {
    #[cold]
    fn drop_slow(&mut self) {
        let hash = self.storage.hasher().hash_one(&*self.value);
        let index = self.storage.determine_shard(hash as usize);
        let shard = &self.storage.shards()[index];

        let value = {
            let mut shard = shard.write();

            // Another thread has interned another copy
            if Arc::strong_count(&self.value) != 2 {
                return;
            }

            // Just compare address
            let value = shard.remove_entry(hash, |v| Arc::ptr_eq(&v.0, &self.value));

            // Shrink the backing storage if the shard is less than 50% occupied.
            let shard_len = shard.len();
            if shard_len * 2 < shard.capacity() {
                shard.shrink_to(shard_len, |v| self.storage.hasher().hash_one(&*v.0));
            }

            value
        };

        drop(value);
    }
}

impl<T: Hash + Eq + 'static> AsRef<T> for Interned<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T: Hash + Eq + 'static> Deref for Interned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Hash + Eq + 'static> Hash for Interned<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // NOTE: Cast disposes vtable pointer / slice/str length.
        state.write_usize(Arc::as_ptr(&self.value) as *const () as usize)
    }
}

/// Compares interned `Ref`s using pointer equality.
impl<T: Hash + Eq + 'static> PartialEq for Interned<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.value, &other.value)
    }
}

impl<T: Hash + Eq + 'static> Eq for Interned<T> {}

impl<T: Hash + Eq + 'static> PartialOrd for Interned<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Hash + Eq + 'static> cmp::Ord for Interned<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        Arc::as_ptr(&self.value)
            .cast::<()>()
            .cmp(&Arc::as_ptr(&other.value).cast::<()>())
    }
}

impl<T: fmt::Debug + Hash + Eq + 'static> fmt::Debug for Interned<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.value.as_ref(), f)
    }
}

impl<T: Hash + Eq + 'static> Clone for Interned<T> {
    fn clone(&self) -> Self {
        Interned {
            storage: self.storage.clone(),
            value: self.value.clone(),
        }
    }
}

impl<T: Hash + Eq + 'static> Drop for Interned<T> {
    fn drop(&mut self) {
        if Arc::strong_count(&self.value) == 2 {
            self.drop_slow();
        }
    }
}

impl<T: Hash + Eq + 'static> Default for InternMap<T> {
    fn default() -> Self {
        InternMap {
            storage: Default::default(),
        }
    }
}
