#include "wrapper.hpp"

#include <string>

int run_frontend_wrapper(char* filename, char* command, Yosys::RTLIL::Design *design) {
    std::string filename_cpp_str = std::string(filename);
    std::string command_cpp_str = std::string(command);
    return Yosys::run_frontend(filename_cpp_str, command_cpp_str, design);
}