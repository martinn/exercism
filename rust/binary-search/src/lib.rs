use std::cmp::Ordering;

pub fn find<V: AsRef<[T]>, T: Ord>(array: V, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mut left = 0;
    let mut right = array.len();

    while left < right {
        let middle = (left + right) / 2;

        match array[middle].cmp(&key) {
            Ordering::Less => left = middle + 1,
            Ordering::Greater => right = middle,
            Ordering::Equal => return Some(middle),
        }
    }

    None
}
