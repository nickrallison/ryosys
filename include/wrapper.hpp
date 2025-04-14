
#include <string>

#include "kernel/yosys.h"
#include "kernel/hashlib.h"
#include "kernel/rtlil.h"

int run_frontend_wrapper(char* filename, char* command, Yosys::RTLIL::Design *design);