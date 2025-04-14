use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::bindings::*;
use crate::helper::str_to_cstr;

pub mod bindings;
mod helper;

fn main() {


    let text = "(
module mh0 (wire x); endmodule
module mh1 (integer x); endmodule
module mh2 (inout integer x); endmodule
module mh3 ([5:0] x); endmodule
module mh4 (var x); endmodule
module mh5 (input x); endmodule
module mh6 (input var x); endmodule
module mh7 (input var integer x); endmodule
module mh8 (output x); endmodule
module mh9 (output var x); endmodule
module mh10(output signed [5:0] x); endmodule
module mh11(output integer x); endmodule
module mh12(ref [5:0] x); endmodule
module mh13(ref x [5:0]); endmodule
module mh14(wire x, y[7:0]); endmodule
module mh15(integer x, signed [5:0] y); endmodule
module mh16([5:0] x, wire y); endmodule
module mh17(input var integer x, wire y); endmodule
module mh18(output var x, input y); endmodule
module mh19(output signed [5:0] x, integer y); endmodule
module mh20(ref [5:0] x, y); endmodule
module mh21(ref x [5:0], y); endmodule
module mh22(ref wire x); endmodule
)";
    let path = "/home/nick/tmp/file.sv";

    // Write the text to the temporary file
    std::fs::write(path, text).expect("Should be able to write file");
    let read_back = std::fs::read_to_string(path);
    assert_eq!(read_back.unwrap(), text.to_string());
    let frontend_command = "auto";
    let design: *mut Yosys_RTLIL_Design = null_mut();
    unsafe { Yosys_yosys_setup(); }

    let frontend_command: *mut c_char = str_to_cstr(frontend_command);
    let file_path: *mut c_char = str_to_cstr(path);

    let result = unsafe { Yosys_run_frontend_wrapper(file_path, frontend_command, design) };

    println!("frontend_result: {result}");

}
