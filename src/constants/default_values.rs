use std::marker::PhantomData;

#[derive(Debug)]
pub struct DefaultValues<'a> {
    /// https://stackoverflow.com/questions/40484154/parameter-a-is-never-used-error-when-a-is-used-in-type-parameter-bound
    // Causes the type to function *as though* it has a `&'a ()` field,
    // despite not *actually* having one.
    _marker: PhantomData<&'a ()>,
}

impl DefaultValues<'static> {
    // To compile the app, make sure to modify the `CHRONO_TZ_TIMEZONE_FILTER` env variable according to your TZ choice. This is done to treeshake the `chrono_tz` crate.
    // eg: cargo run
    pub const DEFAULT_TIMEZONE: chrono_tz::Tz = chrono_tz::Tz::Asia__Kolkata;

    pub const MAX_ACTIVITIES_VECTOR_LENGTH_PER_DEVICE: usize = 1000;
    pub const MAX_ACTIVITIES_PER_DEVICE_CLEAR_SPLICE_SIZE: usize = 50;
}
