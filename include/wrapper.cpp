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

    size_t get_num_monitors(const Yosys::RTLIL::Module* module)  {
        return module->monitors.size();
    }
    Yosys::RTLIL::Monitor* get_monitor_by_index(const Yosys::RTLIL::Module* module, size_t idx) {
        auto it = module->monitors.begin();
        std::advance(it, idx);
        return *it;
    }

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

}



