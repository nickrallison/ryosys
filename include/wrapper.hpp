
#include <string>

#include "kernel/yosys.h"
#include "kernel/hashlib.h"
#include "kernel/rtlil.h"

bool run_frontend_wrapper(char* filename, char* command, Yosys::RTLIL::Design *design);

Yosys::RTLIL::Design* new_yosys_rtlil_design();