
use crate::bindings::{Yosys_RTLIL_Cell};

pub struct Cell {
    // UNSAFE
    ptr: *mut Yosys_RTLIL_Cell,

    // INTERFACE
}

impl Cell {
    pub(crate) fn from_ptr(ptr: *mut Yosys_RTLIL_Cell) -> Self {
        Self {
            ptr
        }
    }
}