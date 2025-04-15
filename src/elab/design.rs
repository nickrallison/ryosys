use std::os::raw::c_char;
use crate::bindings::*;
use crate::elab::module::Module;
use crate::frontend::Frontend;
use crate::helper::str_to_cstr;

pub struct Design {
    // UNSAFE
    design_ptr: *mut Yosys_RTLIL_Design,

    // INTERFACE
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

        let top_module: Module = unsafe {
            let ptr = Yosys_RTLIL_Design_top_module(design_ptr);
            Module::from_ptr(ptr)
        };

        Self {
            design_ptr,
            top_module
        }
    }

    pub fn get_top_module(&self) -> &Module {
        &self.top_module
    }

    fn reform_design(&mut self){
        self.top_module = unsafe {
            let ptr = Yosys_RTLIL_Design_top_module(self.design_ptr);
            Module::from_ptr(ptr)
        };
    }
}

impl Drop for Design {
    fn drop(&mut self) {
        unsafe {
            Wrapper_delete_yosys_rtlil_design(self.design_ptr);
        };
    }
}
