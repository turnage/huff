//! heap provides Heap implementations;

pub mod binary;

use std::cmp::Ordering;
use std::fmt::Display;
use data::Elem;

/// Heap holds key value pairs and always pops the element with the highest value key which was
/// inserted first.
///
/// Worst cases
///     Insertion O(log(n))
///     Pop O(log(n))
pub trait Heap<K: Ord, V> {
    fn insert(&mut self, k: K, v: V);
    fn pop(&mut self) -> Option<V>;
}

impl<K: Ord, V> Iterator for Heap<K, V> {
    type Item = V;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

trait HeapTest<K: Ord, V>: Heap<K, V> {
    fn validate(&self) {
        self.validate_node(0);
    }

    fn validate_node(&self, n: usize);
}

struct Node<K: Ord, V> {
    elem: Elem<K, V>,
    order: usize,
}

impl<K: Ord, V> Ord for Node<K, V> {
    fn cmp(&self, other: &Node<K, V>) -> Ordering {
        match self.elem.cmp(&other.elem) {
            Ordering::Equal => self.order.cmp(&other.order),
            o => o,
        }
    }
}

impl<K: Ord, V> PartialOrd for Node<K, V> {
    fn partial_cmp(&self, other: &Node<K, V>) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl<K: Ord, V> Eq for Node<K, V> {}

impl<K: Ord, V> PartialEq for Node<K, V> {
    fn eq(&self, other: &Node<K, V>) -> bool {
        self.elem == other.elem
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> Vec<Node<usize, usize>> {
        vec![Node {
                 elem: Elem { key: 1, val: 4 },
                 order: 0,
             },
             Node {
                 elem: Elem { key: 1, val: 5 },
                 order: 1,
             },
             Node {
                 elem: Elem { key: 3, val: 1 },
                 order: 2,
             },
             Node {
                 elem: Elem { key: 4, val: 0 },
                 order: 3,
             },
             Node {
                 elem: Elem { key: 2, val: 2 },
                 order: 4,
             },
             Node {
                 elem: Elem { key: 2, val: 3 },
                 order: 5,
             }]
    }

    fn test_heap<K, V, H>(mut h: H, ns: Vec<Node<K, V>>)
        where K: Ord + Copy + Display,
              V: Ord + Copy + Display,
              H: Heap<K, V> + HeapTest<K, V>
    {
        for n in ns.iter() {
            h.insert(n.elem.key, n.elem.val);
        }
        h.validate();

        let mut extracted = 0;
        let mut last_value = ns.iter().map(|n| n.elem.val).min().unwrap();
        while let Some(v) = h.pop() {
            assert!(v >= last_value);
            last_value = v;
            extracted += 1;
            h.validate();
        }
        assert_eq!(extracted, ns.len());
    }

    #[test]
    fn binary_heap() {
        test_heap(binary::BinaryHeap::max(), input());
    }
}
