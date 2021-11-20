use crate::max_of;
use std::fmt::Debug;
use std::num::Wrapping;

pub fn truncate_from_last<T>(vec: Vec<T>, max_size_inclusive: usize) -> Vec<T>
where
    T: Clone,
    T: Debug,
{
    let _vec = vec;
    let vec_length = _vec.len();

    if vec_length <= max_size_inclusive {
        return _vec;
    }

    let max_size_inclusive = Wrapping(max_size_inclusive);
    let vec_length = Wrapping(vec_length);

    let split_off_index = max_of(vec_length - max_size_inclusive, Wrapping(0));

    let o = _vec.split_at(split_off_index);

    o.1.to_vec()
}

pub fn push_to_last_and_maintain_capacity_of_vector<T>(
    vec: Vec<T>,
    max_size_inclusive: usize,
    item: T,
) -> Vec<T>
where
    T: Clone,
    T: Debug,
{
    let mut _vec = vec;
    _vec.push(item);

    truncate_from_last(_vec, max_size_inclusive)
}
