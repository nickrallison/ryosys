
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
    // -- MONITORS
    size_t get_num_monitors(const Yosys::RTLIL::Module* module);
    Yosys::RTLIL::Monitor* get_monitor_by_index(const Yosys::RTLIL::Module* module, size_t idx);

    // -- WIRES
    size_t get_num_wires(const Yosys::RTLIL::Module* module);
    Yosys::RTLIL::Wire* get_wire_by_index(const Yosys::RTLIL::Module* module, size_t idx);
    Yosys::RTLIL::IdString get_wire_id_by_index(const Yosys::RTLIL::Module* module, size_t idx);

    // -- CELLS
    size_t get_num_cells(const Yosys::RTLIL::Module* module);
    Yosys::RTLIL::Cell* get_cell_by_index(const Yosys::RTLIL::Module* module, size_t idx);
    Yosys::RTLIL::IdString get_cell_id_by_index(const Yosys::RTLIL::Module* module, size_t idx);

    // -- CONNECTIONS
    size_t get_num_connections(const Yosys::RTLIL::Module* module);
    Yosys::RTLIL::SigSig get_connection_by_index(const Yosys::RTLIL::Module* module, size_t idx);

    // -- BINDINGS
    size_t get_num_bindings(const Yosys::RTLIL::Module* module);
    Yosys::RTLIL::Binding* get_binding_by_index(const Yosys::RTLIL::Module* module, size_t idx);

    // -- AVAILABLE PARAMS
    size_t get_num_available_params(const Yosys::RTLIL::Module* module);
    Yosys::RTLIL::IdString get_available_param_by_index(const Yosys::RTLIL::Module* module, size_t idx);

    // -- PARAM DEFAULTS
    size_t get_num_parameter_defaults(const Yosys::RTLIL::Module* module);
    Yosys::RTLIL::Const get_parameter_default_by_index(const Yosys::RTLIL::Module* module, size_t idx);
    Yosys::RTLIL::IdString get_parameter_default_id_by_index(const Yosys::RTLIL::Module* module, size_t idx);

    // -- MEMORIES
    size_t get_num_memories(const Yosys::RTLIL::Module* module);
    Yosys::RTLIL::Memory* get_memory_by_index(const Yosys::RTLIL::Module* module, size_t idx);
    Yosys::RTLIL::IdString get_memory_id_by_index(const Yosys::RTLIL::Module* module, size_t idx);

    // -- PROCESSES
    size_t get_num_processes(const Yosys::RTLIL::Module* module);
    Yosys::RTLIL::Process* get_process_by_index(const Yosys::RTLIL::Module* module, size_t idx);
    Yosys::RTLIL::IdString get_process_id_by_index(const Yosys::RTLIL::Module* module, size_t idx);
}