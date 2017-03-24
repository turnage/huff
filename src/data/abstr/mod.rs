use data::{Countable, Peekable};

pub trait PriorityQueue<T: Ord>: Peekable<T> + Countable {
    fn enqueue(&mut self, e: T);
    fn dequeue(&mut self) -> Option<T>;
}
