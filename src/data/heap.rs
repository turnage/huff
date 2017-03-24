use std::cmp::max;

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

struct node<K: Ord, V> {
    key: K,
    val: V,
}

pub struct BinaryHeap<K: Ord, V> {
    state: Vec<node<K, V>>,
}

impl<K: Ord, V> BinaryHeap<K, V> {
    fn max() -> Self {
        BinaryHeap { state: Vec::new() }
    }

    fn inconsistent_child(&self, i: usize) -> Option<usize> {
        if let Some(c) = self.largest_child(i) {
            if self.key(i) < self.key(c) {
                self.largest_child(i)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn largest_child(&self, i: usize) -> Option<usize> {
        let left = self.key(self.left(i));
        let right = self.key(self.right(i));
        if left > right {
            Some(self.left(i))
        } else if self.exists(self.right(i)) {
            Some(self.right(i))
        } else {
            None
        }
    }

    fn key(&self, i: usize) -> Option<&K> {
        if self.exists(i) {
            Some(&self.state[i].key)
        } else {
            None
        }
    }

    fn lt_parent(&self, i: usize) -> bool {
        self.is_root(i) || self.state[i].key < self.state[self.parent(i)].key
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
        self.state.push(node { key: k, val: v });
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
                n = c
            }
            Some(v.val)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn tuple_of<K: Ord, V>(n: node<K, V>) -> (K, V) {
        (n.key, n.val)
    }

    #[test]
    fn binary_heap() {
        let pairs = &[node { key: 1, val: 4 },
                      node { key: 1, val: 5 },
                      node { key: 3, val: 1 },
                      node { key: 2, val: 2 },
                      node { key: 2, val: 3 },
                      node { key: 0, val: 6 }];
        let mut bh = BinaryHeap::max();

        for p in pairs {
            bh.insert(p.key, p.val);
        }

        let mut extracted = 0;
        let mut last_value = 0;
        while let Some(v) = bh.pop() {
            assert!(v > last_value);
            extracted += 1;
        }
        assert_eq!(extracted, pairs.len());
    }
}
