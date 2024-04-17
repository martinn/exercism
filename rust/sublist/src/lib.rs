#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    match (first, second) {
        (a, b) if a == b => Comparison::Equal,
        (a, b) if a.includes(b) => Comparison::Superlist,
        (a, b) if b.includes(a) => Comparison::Sublist,
        _ => Comparison::Unequal,
    }
}

pub trait Includes: PartialEq {
    fn includes(&self, other: &Self) -> bool;
}

impl<T: PartialEq> Includes for [T] {
    fn includes(&self, other: &Self) -> bool {
        let size = other.len();
        size == 0 || self.windows(size).any(|w| w == other)
    }
}
