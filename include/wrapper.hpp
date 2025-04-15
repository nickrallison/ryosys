
#include <string>
#include <vector>

#include "kernel/yosys.h"
#include "kernel/hashlib.h"
#include "kernel/rtlil.h"



namespace Wrapper {

    // In a custom header for bindgen:
    using RTLIL_Wires = Yosys::hashlib::dict<Yosys::RTLIL::IdString, Yosys::RTLIL::Wire*, Yosys::hashlib::hash_ops<Yosys::RTLIL::IdString>>;
//    using RTLIL_ObjPool = hashlib::pool<RTLIL::Obj>;

    void run_frontend_wrapper(char* filename, char* command, Yosys::RTLIL::Design *design);
    Yosys::RTLIL::Design* new_yosys_rtlil_design();
    void delete_yosys_rtlil_design(Yosys::RTLIL::Design* design);

    // MODULE
    size_t get_num_wires(const Yosys::RTLIL::Module* module);
    Yosys::RTLIL::Wire* get_wire_by_index(const Yosys::RTLIL::Module* module, size_t idx);
    size_t get_num_cells(const Yosys::RTLIL::Module* module);
    Yosys::RTLIL::Cell* get_cell_by_index(const Yosys::RTLIL::Module* module, size_t idx);
}