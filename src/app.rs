use std::path::{Path, PathBuf};

#[derive(Default)]
pub enum Status {
    #[default]
    Idle,
    Success(String),
    Error(String),
}

#[derive(Default)]
pub struct App {
    pub markdown: String,
    pub status: Status,
    pub source_path: Option<PathBuf>,
}

impl App {
    pub fn convert_pdf(&mut self, path: &Path) {
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                self.status = Status::Error(format!("Lecture impossible : {}", e));
                return;
            }
        };

        match pdf_extract::extract_text_from_mem(&bytes) {
            Ok(text) => {
                self.markdown = text;
                self.source_path = Some(path.to_path_buf());
                self.status = Status::Success(format!("Converti : {}", path.display()));
            }
            Err(e) => {
                self.status = Status::Error(format!("Extraction PDF échouée : {}", e));
            }
        }
    }

    pub fn default_output_name(&self) -> String {
        if let Some(path) = &self.source_path {
            if let Some(stem) = path.file_stem() {
                return format!("{}.md", stem.to_string_lossy());
            }
        }
        "output.md".to_string()
    }
}
