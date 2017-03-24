pub mod heap;
pub mod elem;
pub mod abstr;

pub trait Countable {
    fn len(&self) -> usize;
}

pub trait Peekable<T> {
    fn peek(&self) -> Option<&T>;
}
