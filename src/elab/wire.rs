
use crate::bindings::{Yosys_RTLIL_SigSig, Yosys_RTLIL_Wire};
use crate::elab::sig_sig::SigSig;

pub struct Wire {
    // UNSAFE
    ptr: *mut Yosys_RTLIL_Wire,

    // INTERFACE
}

impl Wire {

}

impl From<*mut Yosys_RTLIL_Wire> for Wire {
    fn from(ptr: *mut Yosys_RTLIL_Wire) -> Self {
        Self {
            ptr,
        }
    }
}