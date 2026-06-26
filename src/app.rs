use std::path::PathBuf;
use std::process::Command;

pub enum Status {
    Idle,
    Success(String),
    Error(String),
}

pub struct App {
    pub markdown: String,
    pub status: Status,
    pub source_path: Option<PathBuf>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            markdown: String::new(),
            status: Status::Idle,
            source_path: None,
        }
    }
}

impl App {
    pub fn convert_pdf(&mut self, path: &PathBuf) {
        let result = Command::new("pdftotext")
            .arg(path)
            .arg("-")
            .output();

        match result {
            Ok(output) => {
                if output.status.success() {
                    self.markdown = String::from_utf8_lossy(&output.stdout).into_owned();
                    self.source_path = Some(path.clone());
                    self.status = Status::Success(format!("Converti : {}", path.display()));
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    self.status = Status::Error(format!("pdftotext : {}", stderr));
                }
            }
            Err(e) => {
                self.status = Status::Error(format!(
                    "pdftotext introuvable. Installe poppler : brew install poppler ({})",
                    e
                ));
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
