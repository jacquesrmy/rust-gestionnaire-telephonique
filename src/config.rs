use std::path::Path;

use crate::error::AppError;

pub fn build_output_path(input_path: &str) -> Result<String, AppError> {
    let path = Path::new(input_path);

    let file_stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or(AppError::InvalidFile(input_path.to_string()))?;

    Ok(format!("graph/{file_stem}.puml"))
}
