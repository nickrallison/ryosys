
use crate::bindings::{Yosys_RTLIL_Wire};

pub struct Wire {
    // UNSAFE
    ptr: *mut Yosys_RTLIL_Wire,

    // INTERFACE
}

impl Wire {
    pub(crate) fn from_ptr(ptr: *mut Yosys_RTLIL_Wire) -> Self {
        Self {
            ptr
        }
    }
}