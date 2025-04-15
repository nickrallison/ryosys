
use crate::bindings::{Yosys_RTLIL_Monitor, Yosys_RTLIL_Process};
use crate::elab::monitor::Monitor;

pub struct Process {
    // UNSAFE
    ptr: *mut Yosys_RTLIL_Process,

    // INTERFACE
}

impl Process {

}

impl From<*mut Yosys_RTLIL_Process> for Process {
    fn from(ptr: *mut Yosys_RTLIL_Process) -> Self {
        Self {
            ptr
        }
    }
}