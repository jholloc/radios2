use std::fmt::{Display, Formatter};
use std::error::Error;
use std::ffi::c_uint;
use crate::ErrorCode;

#[derive(Debug)]
pub struct Adios2Error {
    code: ErrorCode,
    msg: String,
}

impl Display for Adios2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str: &'static str = self.code.into();
        write!(f, "Adios2 Error ({}): {}", str, self.msg)
    }
}

impl Error for Adios2Error {}

impl Adios2Error {
    pub(crate) fn new(code: c_uint, msg: &str) -> Self {
        let code = match num::FromPrimitive::from_u32(code) {
            Some(code) => code,
            None => ErrorCode::Unknown,
        };
        Adios2Error {
            code,
            msg: msg.to_owned(),
        }
    }

    pub(crate) fn wrapper(msg: &str) -> Self {
        Adios2Error {
            code: ErrorCode::Wrapper,
            msg: msg.to_owned(),
        }
    }
}
