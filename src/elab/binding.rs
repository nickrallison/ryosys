
use crate::bindings::{Yosys_RTLIL_Binding};

pub struct Binding {
    // UNSAFE
    ptr: *mut Yosys_RTLIL_Binding,

    // INTERFACE
}

impl Binding {

}

impl From<*mut Yosys_RTLIL_Binding> for Binding {
    fn from(ptr: *mut Yosys_RTLIL_Binding) -> Self {
        Self {
            ptr
        }
    }
}