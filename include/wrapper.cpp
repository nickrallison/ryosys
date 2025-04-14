#include "wrapper.hpp"

#include <string>

std::string std_string_from_cstr(char* cstr) {
    return std::string(cstr);
}