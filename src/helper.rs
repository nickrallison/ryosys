use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::bindings::*;

pub(crate) fn str_to_cstr(s: &str) -> *mut c_char {
    let bytes: Vec<u8> = String::from(s).into_bytes();
    let mut c_chars: Vec<i8> = bytes.iter().map(| c | *c as i8).collect::<Vec<i8>>();
    c_chars.push(0); // null terminator
    let ptr: *mut c_char = c_chars.as_mut_ptr();
    std::mem::forget(c_chars);
    return ptr;
}