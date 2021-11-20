use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiErrors<'a> {
    #[error("a deserializer error has occured: {0:?}: {1:?}")]
    _EnumDeserializer(&'a str, &'a str),
}
