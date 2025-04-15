
use crate::bindings::{Yosys_RTLIL_Cell, Yosys_RTLIL_Const};
use crate::elab::cell::Cell;

pub struct Const {
    // UNSAFE
    ptr: *mut Yosys_RTLIL_Const,

    // INTERFACE
}

impl Const {

}

impl From<*mut Yosys_RTLIL_Const> for Const {
    fn from(ptr: *mut Yosys_RTLIL_Const) -> Self {
        Self {
            ptr
        }
    }
}