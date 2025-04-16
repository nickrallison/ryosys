
use crate::bindings::{Wrapper_get_cell_by_index, Wrapper_get_num_cells, Wrapper_get_num_wires, Wrapper_get_wire_by_index, Yosys_RTLIL_Memory, Yosys_RTLIL_Module, Yosys_RTLIL_Module_Add};
use crate::elab::binding::Binding;
use crate::elab::cell::Cell;
use crate::elab::constant::Const;
use crate::elab::helper::dict::Dict;
use crate::elab::helper::id_string::IdString;
use crate::elab::helper::idict::IDict;
use crate::elab::helper::pool::Pool;
use crate::elab::memory::Memory;
use crate::elab::monitor::Monitor;
use crate::elab::process::Process;
use crate::elab::sig_sig::SigSig;
use crate::elab::wire::Wire;

pub struct Module {
    // UNSAFE
    ptr: *mut Yosys_RTLIL_Module,

    // INTERFACE
//     monitors: Pool<Monitor>,
//
//     wires: Dict<IdString, Wire>,
//     cells: Dict<IdString, Cell>,
//
//     connections: Vec<SigSig>,
//     bindings: Vec<Binding>,
//
//     available_parameters: IDict<IdString>,
//     parameter_default_values: Dict<IdString, Const>,
//     memories: Dict<IdString, Memory>,
//     processes: Dict<IdString, Process>,
}

impl Module {

}

impl From<*mut Yosys_RTLIL_Module> for Module {
    fn from(ptr: *mut Yosys_RTLIL_Module) -> Self {
        Self {
            ptr
        }
    }
}