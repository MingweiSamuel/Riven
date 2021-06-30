// use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use parking_lot::Mutex;

pub struct InsertOnlyCHashMap<K: Hash + Eq, V> {
    base: Mutex<HashMap<K, Arc<V>>>,
}

impl<K: Hash + Eq, V> InsertOnlyCHashMap<K, V> {
    #[inline]
    pub fn new() -> Self {
        Self {
            base: Mutex::new(HashMap::new())
        }
    }

    // #[inline]
    // pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<Arc<V>>
    // where
    //     K: Borrow<Q>,
    //     Q: Hash + Eq,
    // {
    //     self.base.lock().get(key).map(|v| Arc::clone(v))
    // }

    #[inline]
    pub fn get_or_insert_with<F: FnOnce() -> V>(&self, key: K, default: F) -> Arc<V>
    {
        Arc::clone(self.base.lock()
            .entry(key)
            .or_insert_with(|| Arc::new(default())))
    }
}
