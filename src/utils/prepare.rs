// src/utils/prepare

use std::fs;
use std::path::Path;

use crate::error::AppError;

// prepared directory
pub fn prepare_output_dir(dir_path: &str) -> Result<(), AppError> {
    let path = Path::new(dir_path);
    if !path.exists() {
        fs::create_dir_all(path)?;
        println!("Created output directory: {}", dir_path);
    }
    Ok(())
}

// 
pub fn ensure_dir(dir: &str) -> Result<(), AppError> {
    let path = std::path::Path::new(dir);
    if !path.exists() {
        std::fs::create_dir(path)?; 
    }
    Ok(())
}
