//! binary provides a Binary Heap.

use std::usize;
use std::cmp;
use std::fmt::Debug;

use data::{Countable, Peekable};
use data::abstr::PriorityQueue;
use data::elem::OrdElem;
use data::heap::{Heap, Property, HeapTest};

/// BinaryHeap maintains a binary tree...
///   - whose root is the maximum element, and among elements of equal value, is the first
///     inserted.
///   - which is complete.
pub struct BinaryHeap<T: Ord> {
    state: Vec<OrdElem<T>>,

    // order decrements with each insertion to give elements of equal value a strict order.
    order: usize,

    prop: Property,
}

impl<T: Ord> BinaryHeap<T> {
    pub fn max() -> Self {
        BinaryHeap {
            state: Vec::new(),
            order: usize::MAX,
            prop: Property::Max,
        }
    }

    pub fn min() -> Self {
        BinaryHeap {
            state: Vec::new(),
            order: usize::MIN,
            prop: Property::Min,
        }
    }

    fn inconsistent_child(&self, i: usize) -> Option<usize> {
        if let Some(c) = self.eager_child(i) {
            if !self.valid_parent(c) { Some(c) } else { None }
        } else {
            None
        }
    }

    fn eager_child(&self, i: usize) -> Option<usize> {
        self.eager_of(self.left(i), self.right(i))
    }

    fn eager_of(&self, a: Option<usize>, b: Option<usize>) -> Option<usize> {
        let akey = a.map(|v| self.key(v));
        let bkey = b.map(|v| self.key(v));
        let keymatch = |k| if k == akey { a } else { b };
        match self.prop {
            Property::Min => keymatch(cmp::min(akey, bkey)),
            Property::Max => keymatch(cmp::max(akey, bkey)),
        }
    }

    fn valid_parent(&self, i: usize) -> bool {
        if let Some(p) = self.parent(i) {
            match self.prop {
                Property::Min => self.state[i] >= self.state[p],
                Property::Max => self.state[i] <= self.state[p],
            }
        } else {
            true
        }
    }

    fn is_root(&self, i: usize) -> bool {
        i == 0
    }

    fn parent(&self, i: usize) -> Option<usize> {
        if self.is_root(i) {
            None
        } else if i % 2 == 0 {
            self.elem((i - 2) / 2)
        } else {
            self.elem((i - 1) / 2)
        }
    }

    fn left(&self, i: usize) -> Option<usize> {
        self.elem(i * 2 + 1)
    }

    fn right(&self, i: usize) -> Option<usize> {
        self.elem(i * 2 + 2)
    }

    fn elem(&self, i: usize) -> Option<usize> {
        if self.exists(i) { Some(i) } else { None }
    }

    fn key(&self, i: usize) -> Option<&T> {
        if let Some(i) = self.elem(i) {
            Some(self.state[i].elem())
        } else {
            None
        }
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
        while !self.is_root(n) && !self.valid_parent(n) {
            let parent = self.parent(n).unwrap();
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

impl<T: Ord + Debug> HeapTest<T> for BinaryHeap<T> {
    fn validate_node(&self, n: usize) {
        if !self.exists(n) {
            return;
        }

        assert_eq!(self.inconsistent_child(n), None);
        assert!(self.valid_parent(n));

        let children = &[self.left(n), self.right(n)];
        let mut citer = children.iter().filter(|c| c.is_some()).map(|c| c.unwrap());
        while let Some(c) = citer.next() {
            self.validate_node(c);
        }
    }
}
