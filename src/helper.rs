use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::bindings::*;

pub(crate) fn str_to_cstr(s: &str) -> *mut c_char {
    let c_string = CString::new(s).expect("CString::new failed");
    let ptr: *mut c_char = c_string.as_ptr() as *mut c_char;
    ptr
}