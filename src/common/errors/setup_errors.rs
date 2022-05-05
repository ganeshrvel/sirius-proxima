use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SetupError<'a> {
    #[error("[1:?] a constant error has occured: {0:?}")]
    Constants(&'a str, &'a str),

    #[error("a setting file error has occured: {0:?}")]
    _Settings(&'a str),

    #[error("[2:?] An error occurred while reading the {0:?} log file: {1:?}")]
    LogFile(&'a str, io::Error, &'a str),

    #[error("[1:?] failed to initialize the logger: {0:?}")]
    _LoggerError(anyhow::Error, &'a str),

    #[error("[2:?] An error occurred while reading the {0:?} file: {1:?}")]
    #[allow(dead_code)]
    SettingsFile(&'a str, io::Error, &'a str),

    #[error("[2:?] An error occurred while deserializing the {0:?} file: {1:?}")]
    SettingsFileDeserialize(&'a str, serde_yaml::Error, &'a str),

    #[error("[2:?] A system time error occurred {0:?}")]
    SystemTime(&'a str, &'a str),
}
