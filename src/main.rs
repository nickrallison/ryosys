use crate::bindings::*;
use crate::helper::str_to_cstr;
use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::elab::design::Design;
use crate::frontend::Frontend;

pub mod bindings;
mod elab;
mod frontend;
mod helper;

fn main() {
    // let path = "/home/nick/Downloads/ryoys/tests/TimerBug.sv";
    let path = "/home/nick/Downloads/ryosys/tests/synth.ys";
    let frontend = Frontend::new(path.parse().unwrap());
    let design = Design::new(frontend);
}
