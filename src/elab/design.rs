use std::os::raw::c_char;
use crate::bindings::*;
use crate::elab::constant::Const;
use crate::elab::module::Module;
use crate::frontend::Frontend;
use crate::helper::str_to_cstr;

pub struct Design {
    // UNSAFE
    design_ptr: *mut Yosys_RTLIL_Design,

    // // INTERFACE
    top_module: Module
}

impl Design {
    pub fn new(frontend: Frontend) -> Design {
        let design_ptr: *mut Yosys_RTLIL_Design = unsafe { Wrapper_new_yosys_rtlil_design() };
        unsafe {
            Yosys_yosys_setup();
        }

        let frontend_command: *mut c_char = str_to_cstr(frontend.get_frontend_command());
        let file_path: *mut c_char = str_to_cstr(frontend.get_sv_file().to_str().unwrap());

        unsafe { Wrapper_run_frontend_wrapper(file_path, frontend_command, design_ptr) };

        Design::from(design_ptr)
    }

    pub fn reload(self) -> Self {
        Design::from(self.design_ptr)
    }
}

impl Drop for Design {
    fn drop(&mut self) {
        unsafe {
            Wrapper_delete_yosys_rtlil_design(self.design_ptr);
        };
    }
}

impl From<*mut Yosys_RTLIL_Design> for Design {
    fn from(ptr: *mut Yosys_RTLIL_Design) -> Self {
        let top_module: Module = unsafe {
            let ptr = Yosys_RTLIL_Design_top_module(ptr);
            Module::from(ptr)
        };

        Self {
            design_ptr: ptr,
            top_module
        }
    }
}