
use crate::bindings::{Wrapper_get_cell_by_index, Wrapper_get_num_cells, Wrapper_get_num_wires, Wrapper_get_wire_by_index, Yosys_RTLIL_Module, Yosys_RTLIL_Module_Add};
use crate::elab::cell::Cell;
use crate::elab::wire::Wire;

pub struct Module {
    // UNSAFE
    ptr: *mut Yosys_RTLIL_Module,

    // INTERFACE
    wires: Vec<Wire>,
    cells: Vec<Cell>,
}

impl Module {
    pub(crate) fn from_ptr(ptr: *mut Yosys_RTLIL_Module) -> Self {
        let len_wires: usize = unsafe {
            Wrapper_get_num_wires(ptr)
        };
        let len_cells: usize = unsafe {
            Wrapper_get_num_cells(ptr)
        };

        let mut wires: Vec<Wire> = Vec::with_capacity(len_wires);
        let mut cells: Vec<Cell> = Vec::with_capacity(len_cells);

        for index in 0..len_wires {
            let wire_ptr = unsafe {
                Wrapper_get_wire_by_index(ptr, index)
            };
            let wire: Wire = Wire::from_ptr(wire_ptr);
            wires.push(wire);
        }

        for index in 0..len_cells {
            let cell_ptr = unsafe {
                Wrapper_get_cell_by_index(ptr, index)
            };
            let cell: Cell = Cell::from_ptr(cell_ptr);
            cells.push(cell);
        }


        Self {
            ptr,
            wires,
            cells
        }
    }
}