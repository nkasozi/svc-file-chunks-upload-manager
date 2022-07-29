use serde::{Deserialize, Serialize};
use std::{
    fmt,
    io::{Error, ErrorKind},
};

#[derive(Serialize, Deserialize, Debug)]
pub enum AppErrorKind {
    NotFound,
    InternalError,
    ConnectionError,
    ResponseUnmarshalError,
    BadClientRequest,
    ServiceResponseError,
}

impl AppErrorKind {
    pub fn as_str(&self) -> &'static str {
        use AppErrorKind::*;
        // Strictly alphabetical, please.  (Sadly rustfmt cannot do this yet.)
        match *self {
            NotFound => "TaskNotFound",
            InternalError => "InternalError",
            ConnectionError => "ConnectionError",
            ResponseUnmarshalError => "ResponseUnmarshalError",
            BadClientRequest => "BadClientRequest",
            ServiceResponseError => "ServiceResponseError",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppError {
    pub kind: AppErrorKind,
    pub message: String,
}

impl AppError {
    pub fn new(kind: AppErrorKind, message: String) -> AppError {
        AppError { kind, message }
    }
}

// Different error messages according to AppError.code
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - [{}]", self.kind.as_str(), self.message)
    }
}

impl From<AppError> for Error {
    fn from(details: AppError) -> Self {
        return Error::new(ErrorKind::Other, details.message);
    }
}
