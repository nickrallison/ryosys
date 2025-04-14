use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::bindings::*;
use crate::helper::str_to_cstr;

pub mod bindings;
mod helper;
mod frontend;
mod elab;

fn main() {
    
    // let path = "/home/nick/Downloads/ryosys/tests/TimerBug.sv";
    let path = "/home/nick/Downloads/ryosys/tests/synth.ys";

    let frontend_command = "auto";
    let design: *mut Yosys_RTLIL_Design = unsafe { new_yosys_rtlil_design() };
    unsafe { Yosys_yosys_setup(); }

    let frontend_command: *mut c_char = str_to_cstr(frontend_command);
    let file_path: *mut c_char = str_to_cstr(path);

    unsafe { Yosys_run_frontend_wrapper(file_path, frontend_command, design) };

}
