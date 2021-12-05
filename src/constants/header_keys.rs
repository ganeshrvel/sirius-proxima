use crate::constants::app_env::AppEnv;
use std::marker::PhantomData;

#[non_exhaustive]
#[derive(Debug)]
pub struct HeaderKeys<'a> {
    /// https://stackoverflow.com/questions/40484154/parameter-a-is-never-used-error-when-a-is-used-in-type-parameter-bound
    // Causes the type to function *as though* it has a `&'a ()` field,
    // despite not *actually* having one.
    _marker: PhantomData<&'a ()>,
}

impl HeaderKeys<'static> {
    pub const DEVICE_ID: &'static str = "x-device-id";
    pub const CONTENT_TYPE: &'static str = "Content-Type";
}
