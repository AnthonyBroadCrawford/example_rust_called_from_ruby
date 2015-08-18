extern crate libc;

use libc::*;
use std::ffi::{CString};

#[no_mangle]
pub extern "C" fn hello_world() -> *const libc::c_char {
    CString::new("hello world").unwrap().as_ptr()
}
