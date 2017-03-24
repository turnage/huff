//! elem provides element types to put in data structures.

use std::cmp::Ordering;

/// OrdElem holds an ordinal element, and an order value. It implements Ord according to first the
/// element vs element comparison, then the provided order.
pub struct OrdElem<T: Ord> {
    elem: T,
    order: usize,
}

impl<T: Ord> OrdElem<T> {
    pub fn from(e: T, o: usize) -> Self {
        OrdElem {
            elem: e,
            order: o,
        }
    }

    pub fn elem(&self) -> &T {
        &self.elem
    }

    pub fn consume(self) -> T {
        self.elem
    }
}

impl<T: Ord> Ord for OrdElem<T> {
    fn cmp(&self, other: &OrdElem<T>) -> Ordering {
        match self.elem.cmp(&other.elem) {
            Ordering::Equal => self.order.cmp(&other.order),
            o => o,
        }
    }
}

impl<T: Ord> PartialOrd for OrdElem<T> {
    fn partial_cmp(&self, other: &OrdElem<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> Eq for OrdElem<T> {}

impl<T: Ord> PartialEq for OrdElem<T> {
    fn eq(&self, other: &OrdElem<T>) -> bool {
        self.elem == other.elem
    }
}

/// KVPair holds a key and value, and implements Ord according to only the key.
pub struct KVPair<K: Ord, V> {
    key: K,
    val: V,
}

impl<K: Ord, V> KVPair<K, V> {
    pub fn from(k: K, v: V) -> Self {
        KVPair { key: k, val: v }
    }

    pub fn consume(self) -> (K, V) {
        (self.key, self.val)
    }
}

impl<K: Ord, V> Ord for KVPair<K, V> {
    fn cmp(&self, other: &KVPair<K, V>) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K: Ord, V> PartialOrd for KVPair<K, V> {
    fn partial_cmp(&self, other: &KVPair<K, V>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> Eq for KVPair<K, V> {}

impl<K: Ord, V> PartialEq for KVPair<K, V> {
    fn eq(&self, other: &KVPair<K, V>) -> bool {
        self.key == other.key
    }
}
