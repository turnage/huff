pub mod heap;

use std::cmp::Ordering;

struct Elem<K: Ord, V> {
    key: K,
    val: V,
}

impl<K: Ord, V> Ord for Elem<K, V> {
    fn cmp(&self, other: &Elem<K, V>) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K: Ord, V> PartialOrd for Elem<K, V> {
    fn partial_cmp(&self, other: &Elem<K, V>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> Eq for Elem<K, V> {}

impl<K: Ord, V> PartialEq for Elem<K, V> {
    fn eq(&self, other: &Elem<K, V>) -> bool {
        self.key == other.key
    }
}
