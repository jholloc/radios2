#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]

use num_derive::FromPrimitive;
use strum_macros::IntoStaticStr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Write = adios2_mode_adios2_mode_write as isize,
    Read = adios2_mode_adios2_mode_read as isize,
    Append = adios2_mode_adios2_mode_append as isize,
    RandomAccess = adios2_mode_adios2_mode_readRandomAccess as isize,
}

#[derive(Debug, Copy, Clone)]
pub enum ExecutionMode {
    Sync = adios2_mode_adios2_mode_sync as isize,
    Deferred = adios2_mode_adios2_mode_deferred as isize,
}

#[derive(Debug, Copy, Clone)]
pub enum StepMode {
    Append = adios2_step_mode_adios2_step_mode_append as isize,
    Update = adios2_step_mode_adios2_step_mode_update as isize,
    Read = adios2_step_mode_adios2_step_mode_read as isize,
    Undefined,
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum StepStatus {
    Error = adios2_step_status_adios2_step_status_other_error as isize,
    Ok = adios2_step_status_adios2_step_status_ok as isize,
    NotReady = adios2_step_status_adios2_step_status_not_ready as isize,
    EoS = adios2_step_status_adios2_step_status_end_of_stream as isize,
}

#[derive(Debug, Copy, Clone, FromPrimitive, IntoStaticStr)]
pub enum ErrorCode {
    None = adios2_error_adios2_error_none as isize,
    InvalidArgument = adios2_error_adios2_error_invalid_argument as isize,
    SystemError = adios2_error_adios2_error_system_error as isize,
    RuntimeError = adios2_error_adios2_error_runtime_error as isize,
    Exception = adios2_error_adios2_error_exception as isize,
    Unknown,
    Wrapper,
}

pub trait ToAdiosType {
    fn to_adios_type() -> i32;
}

impl ToAdiosType for f32 {
    fn to_adios_type() -> i32 {
        adios2_type_adios2_type_float
    }
}

impl ToAdiosType for f64 {
    fn to_adios_type() -> i32 {
        adios2_type_adios2_type_double
    }
}