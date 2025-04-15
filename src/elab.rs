use crate::bindings::{
    Yosys_RTLIL_Design, Yosys_run_frontend_wrapper, Yosys_yosys_setup, new_yosys_rtlil_design,
};
use crate::frontend::Frontend;
use crate::helper::str_to_cstr;
use std::os::raw::c_char;

pub struct Elab {
    yosys_rtlil_design: *mut Yosys_RTLIL_Design,
}

impl Elab {
    pub fn elab(frontend: Frontend) -> Elab {
        let design: *mut Yosys_RTLIL_Design = unsafe { new_yosys_rtlil_design() };
        unsafe {
            Yosys_yosys_setup();
        }

        let frontend_command: *mut c_char = str_to_cstr(frontend.get_frontend_command());
        let file_path: *mut c_char = str_to_cstr(frontend.get_sv_file().to_str().unwrap());

        unsafe { Yosys_run_frontend_wrapper(file_path, frontend_command, design) };

        Elab {
            yosys_rtlil_design: design,
        }
    }
}
