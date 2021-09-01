use failure::Fail;
use std::io;
use std::io::Error;

#[derive(Fail, Debug)]
pub enum KvsError{
    #[fail(display = "{}", _0)]
    IO(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
    #[fail(display = "key not found")]
    KeyNotFound,
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
}

impl From<io::Error>for KvsError{
    fn from(err: Error) -> KvsError {
        KvsError::IO(err)
    }
}
impl From<serde_json::Error>for KvsError{
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::Serde(err)
    }
}
pub type Result<T> = std::result::Result<T, KvsError>;