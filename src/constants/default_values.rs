use std::marker::PhantomData;

#[derive(Debug)]
pub struct DefaultValues<'a> {
    /// https://stackoverflow.com/questions/40484154/parameter-a-is-never-used-error-when-a-is-used-in-type-parameter-bound
    // Causes the type to function *as though* it has a `&'a ()` field,
    // despite not *actually* having one.
    _marker: PhantomData<&'a ()>,
}

impl DefaultValues<'static> {
    pub const MAX_ACTIVITIES_VECTOR_LENGTH_PER_DEVICE: usize = 1000;
}
