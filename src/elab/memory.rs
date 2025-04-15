
use crate::bindings::{Yosys_RTLIL_Const, Yosys_RTLIL_Memory};
use crate::elab::constant::Const;

pub struct Memory {
    // UNSAFE
    ptr: *mut Yosys_RTLIL_Memory,

    // INTERFACE
}

impl Memory {

}

impl From<*mut Yosys_RTLIL_Memory> for Memory {
    fn from(ptr: *mut Yosys_RTLIL_Memory) -> Self {
        Self {
            ptr
        }
    }
}