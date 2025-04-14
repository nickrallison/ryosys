#include "wrapper.hpp"

#include <string>

bool run_frontend_wrapper(char* filename, char* command, Yosys::RTLIL::Design *design) {
    std::string filename_cpp_str = std::string(filename);
    std::string command_cpp_str = std::string(command);
    auto result = Yosys::run_frontend(filename_cpp_str, command_cpp_str, design);
    return design;
}

Yosys::RTLIL::Design* new_yosys_rtlil_design() {
    return new Yosys::RTLIL::Design;
}