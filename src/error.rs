use std::fmt;

#[derive(Debug)]
pub enum AppError {
    MissingArgument,
    InvalidFile(String),
    JsonError(String),
    IoError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::MissingArgument => write!(f, "Argument manquant"),
            AppError::InvalidFile(path) => write!(f, "Fichier invalide: {}", path),
            AppError::JsonError(e) => write!(f, "Erreur JSON: {}", e),
            AppError::IoError(e) => write!(f, "Erreur IO: {}", e),
        }
    }
}

impl std::error::Error for AppError {}
