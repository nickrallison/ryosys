use std::path::PathBuf;



pub struct Frontend {
    sv_files: Vec<PathBuf>,
}

impl Frontend {
    pub fn new(sv_files: Vec<PathBuf>) -> Self {
        Frontend { sv_files }
    }
}