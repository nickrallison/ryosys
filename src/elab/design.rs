use std::os::raw::c_char;
use crate::bindings::*;
use crate::frontend::Frontend;
use crate::helper::str_to_cstr;

pub struct Design {
    design_ptr: *mut Yosys_RTLIL_Design,
}

impl Design {
    pub fn new(frontend: Frontend) -> Design {
        let design_ptr: *mut Yosys_RTLIL_Design = unsafe { new_yosys_rtlil_design() };
        unsafe {
            Yosys_yosys_setup();
        }

        let frontend_command: *mut c_char = str_to_cstr(frontend.get_frontend_command());
        let file_path: *mut c_char = str_to_cstr(frontend.get_sv_file().to_str().unwrap());

        unsafe { Yosys_run_frontend_wrapper(file_path, frontend_command, design_ptr) };

        Self { design_ptr }
    }
}

// impl Default for Yosys_RTLIL_Design {
//     fn default() -> Self {
//         Self {
//             hashidx_: 0,
//             monitors: Yosys_hashlib_pool {},
//             scratchpad: Yosys_hashlib_dict {},
//             flagBufferedNormalized: false,
//             refcount_modules_: 0,
//             modules_: Yosys_hashlib_dict {},
//             bindings_: Default::default(),
//             verilog_packages: Default::default(),
//             verilog_globals: Default::default(),
//             verilog_defines: 0,
//             selection_stack: Default::default(),
//             selection_vars: Yosys_hashlib_dict {},
//             selected_active_module: Default::default(),
//         }
//     }
// }