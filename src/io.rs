use std::ffi::CString;
use crate::engine::Engine;
use crate::{checked_call, Mode, StepMode, ToAdiosType, Variable};
use crate::errors::Adios2Error;
use crate::Result;

use crate::wrapper::*;

pub struct Io {
    pub(crate) handle: *mut adios2_io,
}

impl Io {
    pub fn set_engine(&mut self, name: &str) -> Result<()> {
        let c_name = CString::new(name).unwrap();
        checked_call! { adios2_set_engine(self.handle, c_name.as_ptr()) }
        Ok(())
    }

    pub fn set_parameter(&mut self, key: &str, value: &str) -> Result<()> {
        let c_key = CString::new(key).unwrap();
        let c_value = CString::new(value).unwrap();

        checked_call! { adios2_set_parameter(self.handle, c_key.as_ptr(), c_value.as_ptr()) }

        Ok(())
    }

    pub fn set_parameters(&mut self, params: &[[&str; 2]]) -> Result<()> {
        for param in params {
            let key = param[0];
            let value = param[1];
            self.set_parameter(key, value)?;
        }
        Ok(())
    }

    pub fn clear_parameters(&self) -> Result<()> {
        checked_call!{ adios2_clear_parameters(self.handle) }
        Ok(())
    }

    pub fn parameters(&self) {
        todo!()
    }

    pub fn open(&self, name: &str, mode: Mode) -> Engine {
        let c_name = CString::new(name).unwrap();
        unsafe {
            Engine {
                handle: adios2_open(self.handle, c_name.as_ptr(), mode as u32),
                mode,
                step_mode: StepMode::Undefined,
                timeout: -1.0,
            }
        }
    }

    pub fn define_variable<T: ToAdiosType>(
        &self,
        name: &str,
        ndims: usize,
        shape: &[usize],
        start: &[usize],
        count: &[usize],
        constant_dims: bool,
    ) -> Variable {
        let c_name = CString::new(name).unwrap();
        let constant_dims = if constant_dims {
            adios2_constant_dims_adios2_constant_dims_true
        } else {
            adios2_constant_dims_adios2_constant_dims_false
        };
        let variable = unsafe {
            adios2_define_variable(
                self.handle,
                c_name.as_ptr(),
                T::to_adios_type(),
                ndims,
                shape.as_ptr(),
                start.as_ptr(),
                count.as_ptr(),
                constant_dims,
            )
        };
        Variable { handle: variable }
    }
}
