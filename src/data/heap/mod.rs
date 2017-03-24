//! heap provides Heap implementations;

pub mod binary;

use std::cmp::Ordering;
use std::fmt::Display;

/// Heap holds key value pairs and always pops the element with the highest value key which was
/// inserted first.
///
/// Worst cases
///     Insertion O(log(n))
///     Pop O(log(n))
pub trait Heap<T: Ord> {
    fn insert(&mut self, e: T);
    fn peek(&self) -> Option<&T>;
    fn pop(&mut self) -> Option<T>;
}

impl<T: Ord> Iterator for Heap<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

trait HeapTest<T: Ord>: Heap<T> {
    fn validate(&self) {
        self.validate_node(0);
    }

    fn validate_node(&self, n: usize);
}

#[cfg(test)]
mod test {
    use super::*;

    /// input returns a set of unique elements.
    fn input() -> Vec<usize> {
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    }

    fn test_heap<T: Ord + Copy + Display, H: Heap<T> + HeapTest<T>>(mut h: H, ns: Vec<T>) {
        for n in ns.iter() {
            h.insert(*n);
        }
        h.validate();

        let mut extracted = 0;
        let mut last_value = *ns.iter().max().unwrap();
        while let Some(v) = h.pop() {
            assert!(v <= last_value);
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