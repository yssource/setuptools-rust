#![feature(proc_macro)]

extern crate pyo3;

use pyo3::prelude::*;
use pyo3::py::modinit;

pub mod built_info {
   include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/// Module documentation string
#[modinit("english")]
fn init(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "hello")]
    fn hello(_py: Python) -> PyResult<()> {
        println!("Hello, world!");
        println!("(by the way, I was built on {})", built_info::RUSTC_VERSION);
        Ok(())
    }

    Ok(())
}
