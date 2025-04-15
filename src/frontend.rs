use std::path::PathBuf;



pub struct Frontend {
    sv_files: Vec<PathBuf>,
    frontend_command: String,
}

impl Frontend {
    pub fn new(sv_files: Vec<PathBuf>) -> Self {
        let frontend_command = "auto";
        Frontend { 
            sv_files ,
            frontend_command: frontend_command.to_string(),
        }
    }
    
    pub fn get_sv_files(&self) -> &Vec<PathBuf> {
        &self.sv_files
    }
    
    pub fn get_frontend_command(&self) -> &str {
        &self.frontend_command
    }
}