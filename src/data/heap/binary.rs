//! binary provides a Binary Heap.

use std::usize;

use data::{Countable, Peekable};
use data::abstr::PriorityQueue;
use data::elem::OrdElem;
use data::heap::{Heap, Property};

/// BinaryHeap maintains a binary tree...
///   - whose root is the maximum element, and among elements of equal value, is the first
///     inserted.
///   - which is complete.
pub struct BinaryHeap<T: Ord> {
    state: Vec<OrdElem<T>>,
    order: Box<Iterator<Item = usize>>,
    prop: Property,
}

impl<T: Ord> BinaryHeap<T> {
    pub fn max() -> Self {
        BinaryHeap {
            state: Vec::new(),
            order: Box::new((1..usize::MAX).rev()),
            prop: Property::Max,
        }
    }

    pub fn min() -> Self {
        BinaryHeap {
            state: Vec::new(),
            order: Box::new((1..)),
            prop: Property::Min,
        }
    }

    fn invalid_child(&self, n: usize) -> Option<usize> {
        let invalid_child = |i| match i {
            i if i < self.state.len() => {
                match self.invalid_parent(i) {
                    Some(_) => Some(i),
                    None => None,
                }
            }
            _ => None,
        };
        let left = invalid_child(n * 2 + 1);
        let right = invalid_child(n * 2 + 2);
        match (left, right) {
            (None, None) => None,
            (None, Some(r)) => Some(r),
            (Some(l), None) => Some(l),
            (Some(l), Some(r)) => {
                match self.valid(l, r) {
                    true => Some(l),
                    false => Some(r),
                }
            }

        }
    }

    fn invalid_parent(&self, n: usize) -> Option<usize> {
        match n {
            n if n > 0 => {
                let parent = (n - 1) / 2;
                if self.valid(parent, n) {
                    None
                } else {
                    Some(parent)
                }
            }
            _ => None,
        }
    }

    fn valid(&self, p: usize, c: usize) -> bool {
        match self.prop {
            Property::Min => self.state[p] < self.state[c],
            Property::Max => self.state[p] > self.state[c],
        }
    }
}

impl<T: Ord> Heap<T> for BinaryHeap<T> {
    fn insert(&mut self, e: T) {
        let order = self.order.next().unwrap();
        self.state.push(OrdElem::from(e, order));
        let mut n = self.state.len() - 1;
        while let Some(p) = self.invalid_parent(n) {
            self.state.swap(n, p);
            n = p;
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.state.is_empty() {
            None
        } else {
            let mut n = 0;
            let e = self.state.swap_remove(n);
            while let Some(c) = self.invalid_child(n) {
                self.state.swap(n, c);
                n = c
            }
            Some(e.consume())
        }
    }
}

impl<T: Ord> Countable for BinaryHeap<T> {
    fn len(&self) -> usize {
        self.state.len()
    }
}

impl<T: Ord> Peekable<T> for BinaryHeap<T> {
    fn peek(&self) -> Option<&T> {
        self.state.first().map(|e| e.elem())
    }
}

impl<T: Ord> PriorityQueue<T> for BinaryHeap<T> {
    fn enqueue(&mut self, e: T) {
        self.insert(e)
    }

    fn dequeue(&mut self) -> Option<T> {
        self.pop()
    }
}
