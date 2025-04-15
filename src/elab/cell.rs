
use crate::bindings::{Yosys_RTLIL_Binding, Yosys_RTLIL_Cell};
use crate::elab::binding::Binding;

pub struct Cell {
    // UNSAFE
    ptr: *mut Yosys_RTLIL_Cell,

    // INTERFACE
}

impl Cell {

}

impl From<*mut Yosys_RTLIL_Cell> for Cell {
    fn from(ptr: *mut Yosys_RTLIL_Cell) -> Self {
        Self {
            ptr
        }
    }
}