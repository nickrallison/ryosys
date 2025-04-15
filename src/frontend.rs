use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Frontend {
    // sv_files: Vec<PathBuf>,
    sv_file: PathBuf,
    frontend_command: String,
}

impl Frontend {
    pub fn new(
        // sv_files: Vec<PathBuf>
        sv_file: PathBuf,
    ) -> Self {
        let frontend_command = "auto";
        Frontend {
            // sv_files ,
            sv_file,
            frontend_command: frontend_command.to_string(),
        }
    }

    // pub fn get_sv_files(&self) -> &Vec<PathBuf> {
    //     &self.sv_files
    // }

    pub fn get_sv_file(&self) -> &PathBuf {
        &self.sv_file
    }

    pub fn get_frontend_command(&self) -> &str {
        &self.frontend_command
    }
}
