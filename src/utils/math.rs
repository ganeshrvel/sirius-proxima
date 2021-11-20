use std::cmp::{max, min};
use std::num::Wrapping;

pub fn max_of<T>(a: Wrapping<T>, b: Wrapping<T>) -> T
where
    T: Ord,
{
    max(a, b).0
}

pub fn min_of<T>(a: Wrapping<T>, b: Wrapping<T>) -> T
where
    T: Ord,
{
    min(a, b).0
}
