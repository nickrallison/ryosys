
use crate::bindings::{Yosys_RTLIL_Module, Yosys_RTLIL_Monitor};
use crate::elab::module::Module;

pub struct Monitor {
    // UNSAFE
    ptr: *mut Yosys_RTLIL_Monitor,

    // INTERFACE
}

impl Monitor{

}

impl From<*mut Yosys_RTLIL_Monitor> for Monitor {
    fn from(ptr: *mut Yosys_RTLIL_Monitor) -> Self {
        Self {
            ptr
        }
    }
}