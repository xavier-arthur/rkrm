pub struct FileNotFoundError { 
    pub message: Option<String>,
    pub path: Option<Box<std::path::PathBuf>>
}

impl std::default::Default for FileNotFoundError {
    fn default() -> Self {
        Self { message: None, path: None }
    }
}