#include "wrapper.hpp"

#include <string>

namespace Wrapper {
    void run_frontend_wrapper(char* filename, char* command, Yosys::RTLIL::Design *design) {
        std::string filename_cpp_str = std::string(filename);
        std::string command_cpp_str = std::string(command);
        Yosys::run_frontend(filename_cpp_str, command_cpp_str, design);
    }

    Yosys::RTLIL::Design* new_yosys_rtlil_design() {
        return new Yosys::RTLIL::Design;
    }

    void delete_yosys_rtlil_design(Yosys::RTLIL::Design* design) {
        delete design;
    }

    // MODULE
    // -- MONITORS
    size_t get_num_monitors(const Yosys::RTLIL::Module* module)  {
        return module->monitors.size();
    }
    Yosys::RTLIL::Monitor* get_monitor_by_index(const Yosys::RTLIL::Module* module, size_t idx) {
        auto it = module->monitors.begin();
        std::advance(it, idx);
        return *it;
    }

    // -- WIRES
    size_t get_num_wires(const Yosys::RTLIL::Module* module) {
        return module->wires_.size();
    }
    Yosys::RTLIL::Wire* get_wire_by_index(const Yosys::RTLIL::Module* module, size_t idx) {
        auto it = module->wires_.begin();
        std::advance(it, idx);
        return it->second;
    }
    Yosys::RTLIL::IdString get_wire_id_by_index(const Yosys::RTLIL::Module* module, size_t idx) {
        auto it = module->wires_.begin();
        std::advance(it, idx);
        return it->first;
    }

    // -- CELLS
    size_t get_num_cells(const Yosys::RTLIL::Module* module) {
        return module->cells_.size();
    }
    Yosys::RTLIL::Cell* get_cell_by_index(const Yosys::RTLIL::Module* module, size_t idx) {
        auto it = module->cells_.begin();
        std::advance(it, idx);
        return it->second;
    }
    Yosys::RTLIL::IdString get_cell_id_by_index(const Yosys::RTLIL::Module* module, size_t idx) {
        auto it = module->cells_.begin();
        std::advance(it, idx);
        return it->first;
    }

    // -- CONNECTIONS
    size_t get_num_connections(const Yosys::RTLIL::Module* module) {
        return module->connections_.size();
    }
    Yosys::RTLIL::SigSig get_connection_by_index(const Yosys::RTLIL::Module* module, size_t idx) {
        return module->connections_[idx];
    }

    // -- BINDINGS
    size_t get_num_bindings(const Yosys::RTLIL::Module* module) {
        return module->bindings_.size();
    }
    Yosys::RTLIL::Binding* get_binding_by_index(const Yosys::RTLIL::Module* module, size_t idx) {
        return module->bindings_[idx];
    }

    // -- AVAILABLE PARAMS
    size_t get_num_available_params(const Yosys::RTLIL::Module* module) {
        return module->avail_parameters.size();
    }
    Yosys::RTLIL::IdString get_available_param_by_index(const Yosys::RTLIL::Module* module, size_t idx) {
        return module->avail_parameters[idx];
    }

    // -- PARAM DEFAULTS
    size_t get_num_parameter_defaults(const Yosys::RTLIL::Module* module) {
        return module->parameter_default_values.size();
    }
    Yosys::RTLIL::Const get_parameter_default_by_index(const Yosys::RTLIL::Module* module, size_t idx) {
        auto it = module->parameter_default_values.begin();
        std::advance(it, idx);
        return it->second;
    }
    Yosys::RTLIL::IdString get_parameter_default_id_by_index(const Yosys::RTLIL::Module* module, size_t idx) {
        auto it = module->parameter_default_values.begin();
        std::advance(it, idx);
        return it->first;
    }

    // -- MEMORIES
    size_t get_num_memories(const Yosys::RTLIL::Module* module) {
        return module->memories.size();
    }
    Yosys::RTLIL::Memory* get_memory_by_index(const Yosys::RTLIL::Module* module, size_t idx) {
        auto it = module->memories.begin();
        std::advance(it, idx);
        return it->second;
    }
    Yosys::RTLIL::IdString get_memory_id_by_index(const Yosys::RTLIL::Module* module, size_t idx) {
        auto it = module->memories.begin();
        std::advance(it, idx);
        return it->first;
    }

    // -- PROCESSES
    size_t get_num_processes(const Yosys::RTLIL::Module* module) {
        return module->processes.size();
    }
    Yosys::RTLIL::Process* get_process_by_index(const Yosys::RTLIL::Module* module, size_t idx) {
        auto it = module->processes.begin();
        std::advance(it, idx);
        return it->second;
    }
    Yosys::RTLIL::IdString get_process_id_by_index(const Yosys::RTLIL::Module* module, size_t idx) {
        auto it = module->processes.begin();
        std::advance(it, idx);
        return it->first;
    }

}



