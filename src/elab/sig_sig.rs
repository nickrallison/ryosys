
use crate::bindings::{Yosys_RTLIL_Process, Yosys_RTLIL_SigSig};
use crate::elab::process::Process;

pub struct SigSig {
    // UNSAFE
    ptr: *mut Yosys_RTLIL_SigSig,

    // INTERFACE
}

impl SigSig {

}

impl From<*mut Yosys_RTLIL_SigSig> for SigSig {
    fn from(ptr: *mut Yosys_RTLIL_SigSig) -> Self {
        Self {
            ptr
        }
    }
}