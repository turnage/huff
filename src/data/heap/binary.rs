//! binary provides a Binary Heap.

use std::usize;

use data::Elem;
use data::heap::{Heap, Node, HeapTest};

/// BinaryHeap maintains a binary tree...
///   1. whose root is the maximum element, and among the elements in the tree of equal value, it is
///      the first inserted.
///   2. which is complete.
pub struct BinaryHeap<K: Ord, V> {
    state: Vec<Node<K, V>>,

    // order decrements after every insertion, infinitely, and the value at insertion is assigned
    // to the inserted element. This maintains an order for elements whose keys have equal value.
    order: usize,
}

impl<K: Ord, V> BinaryHeap<K, V> {
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

    fn node(&self, i: usize) -> Option<&Node<K, V>> {
        if self.exists(i) {
            Some(&self.state[i])
        } else {
            None
        }
    }

    fn lt_parent(&self, i: usize) -> bool {
        self.is_root(i) || self.state[i].elem.key <= self.state[self.parent(i)].elem.key
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

impl<K: Ord, V> Heap<K, V> for BinaryHeap<K, V> {
    fn insert(&mut self, k: K, v: V) {
        self.state.push(Node {
            elem: Elem { key: k, val: v },
            order: self.order,
        });
        self.order -= 1;
        let mut n = self.state.len() - 1;
        while !self.is_root(n) && !self.lt_parent(n) {
            let parent = self.parent(n);
            self.state.swap(n, parent);
            n = parent;
        }
    }

    fn pop(&mut self) -> Option<V> {
        if self.state.len() == 0 {
            None
        } else {
            let mut n = 0;
            let v = self.state.swap_remove(n);
            while let Some(c) = self.inconsistent_child(n) {
                self.state.swap(n, c);
                n = c
            }
            Some(v.elem.val)
        }
    }
}

impl<K: Ord, V> HeapTest<K, V> for BinaryHeap<K, V> {
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
