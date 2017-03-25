//! heap provides Heap implementations;

pub mod binary;

pub use self::binary::*;

use data::{Countable, Peekable};

/// Heap holds key value pairs and always pops the element with the highest value key which was
/// inserted first.
///
/// Worst cases
///     Insertion O(log(n))
///     Pop O(log(n))
pub trait Heap<T: Ord>: Peekable<T> + Countable {
    fn insert(&mut self, e: T);
    fn pop(&mut self) -> Option<T>;
}

impl<T: Ord> Iterator for Heap<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

enum Property {
    Min,
    Max,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::Display;

    fn test_heap<T, H, F>(mut h: H, ns: Vec<T>, verify: F)
        where T: Ord + Copy + Display + Debug,
              H: Heap<T>,
              F: Fn(T, T) -> bool
    {
        for n in ns.iter() {
            h.insert(*n);
        }

        let mut extracted = 0;
        let mut last_value = None;
        while let Some(v) = h.pop() {
            if let Some(prev) = last_value {
                assert!(verify(v, prev));
            }
            last_value = Some(v);
            extracted += 1;
        }
        assert_eq!(extracted, ns.len());
    }

    #[test]
    fn binary_heap() {
        test_heap(binary::BinaryHeap::max(),
                  vec![1, 2, 3, 4, 5],
                  |cur, prev| cur <= prev);
        test_heap(binary::BinaryHeap::min(),
                  vec![1, 2, 3, 3, 4, 5],
                  |cur, prev| cur >= prev);
    }
}
