use crate::bindings::*;

pub mod bindings;

// unsafe extern "C" {
//     #[doc = " Creates a syntax tree by guessing at what might be in the given source snippet.\n @a text is the actual source code text.\n @a name is an optional name to give to the loaded source buffer.\n @a path is an optional path to give to the loaded source buffer.\n @return the created and parsed syntax tree."]
//     #[link_name = "\u{1}?fromText@SyntaxTree@syntax@slang@@SA?AV?$shared_ptr@VSyntaxTree@syntax@slang@@@std@@V?$basic_string_view@DU?$char_traits@D@std@@@5@00@Z"]
//     pub fn slang_syntax_SyntaxTree_fromText(
//         text: std_string_view,
//         name: std_string_view,
//         path: std_string_view,
//     ) -> std_shared_ptr<slang_syntax_SyntaxTree>;
// }

fn main() {


    let mut text = "(
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
    // let string_view = std_string_view {
    //     data: text.as_ptr(),
    //     size: text.len(),
    // };
    // 
    // unsafe {
    //     let mut syntax_tree = slang_syntax_SyntaxTree_fromText(text,  "top", "top.sv");
    //     let root = syntax_tree.rootNode;
    // 
    // }

    println!("Hello, world!");


}
