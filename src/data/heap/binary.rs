//! binary provides a Binary Heap.

use std::usize;

use data::{Countable, Peekable};
use data::abstr::PriorityQueue;
use data::elem::OrdElem;
use data::heap::{Heap, HeapTest};

/// BinaryHeap maintains a binary tree...
///   - whose root is the maximum element, and among elements of equal value, is the first
///     inserted.
///   - which is complete.
pub struct BinaryHeap<T: Ord> {
    state: Vec<OrdElem<T>>,

    // order decrements with each insertion to give elements of equal value a strict order.
    order: usize,
}

impl<T: Ord> BinaryHeap<T> {
    pub fn max() -> Self {
        BinaryHeap {
            state: Vec::new(),
            order: usize::MAX,
        }
    }

    fn inconsistent_child(&self, i: usize) -> Option<usize> {
        let gt_parent = |v| if v > self.node(i).unwrap() {
            Some(v)
        } else {
            None
        };
        let left = self.node(self.left(i)).and_then(&gt_parent);
        let right = self.node(self.right(i)).and_then(&gt_parent);
        if left == None && right == None {
            None
        } else if left == None {
            Some(self.right(i))
        } else if right == None {
            Some(self.left(i))
        } else if left.unwrap() < right.unwrap() {
            Some(self.right(i))
        } else {
            Some(self.left(i))
        }
    }

    fn node(&self, i: usize) -> Option<&T> {
        if self.exists(i) {
            Some(&self.state[i].elem())
        } else {
            None
        }
    }

    fn lt_parent(&self, i: usize) -> bool {
        self.is_root(i) || self.state[i] <= self.state[self.parent(i)]
    }

    fn is_root(&self, i: usize) -> bool {
        i == 0
    }

    fn parent(&self, i: usize) -> usize {
        if i % 2 == 0 { (i - 2) / 2 } else { (i - 1) / 2 }
    }

    fn left(&self, i: usize) -> usize {
        i * 2 + 1
    }

    fn right(&self, i: usize) -> usize {
        i * 2 + 2
    }

    fn exists(&self, i: usize) -> bool {
        i < self.state.len()
    }
}

impl<T: Ord> Heap<T> for BinaryHeap<T> {
    fn insert(&mut self, e: T) {
        self.state.push(OrdElem::from(e, self.order));
        self.order -= 1;
        let mut n = self.state.len() - 1;
        while !self.is_root(n) && !self.lt_parent(n) {
            let parent = self.parent(n);
            self.state.swap(n, parent);
            n = parent;
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.state.is_empty() {
            None
        } else {
            let mut n = 0;
            let e = self.state.swap_remove(n);
            while let Some(c) = self.inconsistent_child(n) {
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

impl<T: Ord> HeapTest<T> for BinaryHeap<T> {
    fn validate_node(&self, n: usize) {
        if !self.exists(n) {
            return;
        }

        assert_eq!(self.inconsistent_child(n), None);
        assert!(self.lt_parent(n));

        self.validate_node(self.left(n));
        self.validate_node(self.right(n));
    }
}
