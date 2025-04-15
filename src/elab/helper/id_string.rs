
use crate::bindings::{Yosys_RTLIL_IdString};

pub struct IdString {
    // UNSAFE
    ptr: *mut Yosys_RTLIL_IdString,

    // INTERFACE
}

impl IdString {
    pub(crate) fn from_ptr(ptr: *mut Yosys_RTLIL_IdString) -> Self {
        Self {
            ptr
        }
    }
}