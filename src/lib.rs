use num::Num;
use std::ffi::CString;
use std::fmt::Display;

mod engine;
mod wrapper;
mod io;
mod errors;

use wrapper::*;
use errors::Adios2Error;
use io::Io;

macro_rules! checked_call {
    { $func:ident($($arg:expr),*) } => {
        let rc = unsafe{ $func($($arg,)*) };
        if rc != 0 {
            let msg = format!("failed to call {}", stringify!($func));
            return Err(Adios2Error::new(rc, &msg));
        }
    }
}

pub(crate) use checked_call;

type Result<T> = std::result::Result<T, Adios2Error>;

pub struct Adios2 {
    handle: *mut adios2_adios,
}

pub struct Variable {
    handle: *mut adios2_variable,
}

impl Adios2 {
    pub fn new() -> Self {
        unsafe {
            Adios2 {
                handle: adios2_init_serial(),
            }
        }
    }

    pub fn declare_io(&self, name: &str) -> Io {
        let c_name = CString::new(name).unwrap();
        unsafe {
            Io {
                handle: adios2_declare_io(self.handle, c_name.as_ptr()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_data<T: Num + From<i32> + From<u32>>(step: u32, count: &[usize]) -> Vec<T> {
        let size = count.into_iter().product();
        (0..size)
            .map(|i| Into::<T>::into(i as u32) * Into::<T>::into(10000) + Into::<T>::into(step))
            .collect()
    }

    fn print_data<T: Display>(data: &Vec<T>, step: usize) {
        print!("Step: {} [", step);
        for el in data {
            print!("{} ", el);
        }
        println!("]");
    }

    #[test]
    fn it_works() -> Result<()> {
        let adios = Adios2::new();
        let mut io = adios.declare_io("whatever");
        io.set_engine("DataMan")?;
        io.set_parameters(&[
            ["IPAddress", "127.0.0.1"],
            ["Port", "12306"],
            ["Timeout", "5"],
            ["RendezvousReaderCount", "1"],
        ])?;

        let mut engine = io.open("HelloDataMan", Mode::Write);

        let nx = 10;
        let ny = 10;
        let steps = 1;

        let count = [nx, ny];
        let start = [nx, 0];
        let shape = [nx, ny];

        let float_array_var =
            io.define_variable::<f64>("FloatArray", 2, &shape, &start, &count, true);

        for i in 0..steps {
            let float_vector = generate_data::<f64>(i, &count);
            engine.begin_step()?;
            engine.put(&float_array_var, &float_vector, ExecutionMode::Deferred)?;
            print_data(&float_vector, engine.current_step()?);
            engine.end_step()?;
        }

        engine.close()?;

        Ok(())
    }
}
