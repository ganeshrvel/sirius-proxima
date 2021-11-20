use crate::utils::math::max_of;
use std::fmt::Debug;
use std::num::Wrapping;

pub fn truncate_from_last<T>(vec: Vec<T>, max_size_inclusive: usize) -> Vec<T>
where
    T: Clone + Debug,
{
    let vec_length = vec.len();

    if vec_length <= max_size_inclusive {
        return vec;
    }

    let max_size_inclusive = Wrapping(max_size_inclusive);
    let vec_length = Wrapping(vec_length);

    let split_off_index = max_of(vec_length - max_size_inclusive, Wrapping(0));

    let o = vec.split_at(split_off_index);

    o.1.to_vec()
}

pub fn push_to_last_and_maintain_capacity_of_vector<T>(
    mut vec: Vec<T>,
    max_size_inclusive: usize,
    item: T,
) -> Vec<T>
where
    T: Clone + Debug,
{
    vec.push(item);

    truncate_from_last(vec, max_size_inclusive)
}
