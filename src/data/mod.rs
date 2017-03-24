pub mod heap;
pub mod elem;
pub mod abstr;

pub trait Peekable<T> {
    fn peek(&self) -> Option<&T>;
}
