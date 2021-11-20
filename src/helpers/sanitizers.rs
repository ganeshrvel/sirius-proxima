use crate::{DefaultValues, SetupError};

pub fn sanitize_constants() -> anyhow::Result<()> {
    if DefaultValues::MAX_ACTIVITIES_VECTOR_LENGTH_PER_DEVICE
        - DefaultValues::MAX_ACTIVITIES_PER_DEVICE_CLEAR_SPLICE_SIZE
        < 1
    {
        return Err(SetupError::Constants("value of 'MAX_ACTIVITIES_VECTOR_LENGTH_PER_DEVICE' should be greater than 'MAX_ACTIVITIES_PER_DEVICE_CLEAR_SPLICE_SIZE'", "P00005a").into());
    }

    Ok(())
}
