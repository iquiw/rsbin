use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

pub fn create_dir_if_missing<P: AsRef<Path>>(path: P) -> Result<()> {
    if path.as_ref().is_dir() {
        Ok(())
    } else {
        Ok(fs::create_dir(&path)
            .with_context(|| format!("Unable to create directory {}", path.as_ref().display()))?)
    }
}

pub fn remove_file_if_exists<P: AsRef<Path>>(path: P) -> Result<()> {
    if path.as_ref().is_file() {
        Ok(fs::remove_file(&path)
            .with_context(|| format!("Unable to delete file {}", path.as_ref().display()))?)
    } else {
        Ok(())
    }
}

pub fn remove_dir_if_exists<P: AsRef<Path>>(path: P) -> Result<()> {
    if path.as_ref().is_dir() {
        Ok(fs::remove_dir_all(&path)
            .with_context(|| format!("Unable to delete directory {}", path.as_ref().display()))?)
    } else {
        Ok(())
    }
}
