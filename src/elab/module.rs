
use crate::bindings::{Yosys_RTLIL_Module, Yosys_RTLIL_Module_Add};
use crate::elab::cell::Cell;
use crate::elab::wire::Wire;

pub struct Module {
    // UNSAFE
    ptr: *mut Yosys_RTLIL_Module,

    // INTERFACE
    // wires: Vec<Wire>,
    // cells: Vec<Cell>,
}

impl Module {
    pub(crate) fn from_ptr(ptr: *mut Yosys_RTLIL_Module) -> Self {
        Self {
            ptr
        }
    }
}