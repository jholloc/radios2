use crate::{checked_call, ExecutionMode, Mode, StepMode, StepStatus, Variable};

use std::ffi::c_void;
use crate::errors::Adios2Error;
use crate::wrapper::*;
use crate::Result;

pub struct Engine {
    pub(crate) handle: *mut adios2_engine,
    pub mode: Mode,
    pub step_mode: StepMode,
    pub timeout: f32,
}

impl Engine {
    pub fn begin_step(&mut self) -> Result<StepStatus> {
        let step_mode = match self.step_mode {
            StepMode::Undefined => match self.mode {
                Mode::Write => StepMode::Append,
                Mode::Append => StepMode::Append,
                Mode::Read => StepMode::Read,
                Mode::RandomAccess => StepMode::Read,
            },
            _ => self.step_mode,
        };
        let mut status = 0;
        checked_call! { adios2_begin_step(
            self.handle,
            step_mode as u32,
            self.timeout,
            &mut status
        ) }
        match num::FromPrimitive::from_i32(status) {
            Some(status) => Ok(status),
            None => {
                let msg = format!("unknown step status value: {}", status);
                Err(Adios2Error::wrapper(&msg))
            }
        }
    }

    pub fn end_step(&mut self) -> Result<()> {
        checked_call! { adios2_end_step(self.handle) };
        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        checked_call! { adios2_close(self.handle) };
        Ok(())
    }

    pub fn current_step(&self) -> Result<usize> {
        let mut usize = 0;
        checked_call! { adios2_current_step(&mut usize, self.handle) };
        Ok(usize)
    }

    pub fn put<T>(&self, variable: &Variable, data: &[T], mode: ExecutionMode) -> Result<()> {
        checked_call! { adios2_put(self.handle, variable.handle, data.as_ptr() as *const c_void, mode as u32) }
        Ok(())
    }

    pub fn get<T>(&self, variable: &Variable, data: &mut [T], mode: ExecutionMode) -> Result<()> {
        checked_call! { adios2_get(self.handle, variable.handle, data.as_mut_ptr() as *mut c_void, mode as u32) }
        Ok(())
    }
}
