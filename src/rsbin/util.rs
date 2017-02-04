use std::fs;
use std::path::Path;

use rsbin::errors::{Error, Result, ResultExt};

pub fn err<T, S: Into<Error>>(msg: S) -> Result<T> {
    Err(msg.into())
}

pub fn create_dir_if_missing<P: AsRef<Path>>(path: P) -> Result<()> {
    if path.as_ref().is_dir() {
        Ok(())
    } else {
        Ok(try!(fs::create_dir(&path)
                .chain_err(|| format!("Unable to create directory {}",
                                      path.as_ref().display()))))
    }
}

pub fn remove_file_if_exists<P: AsRef<Path>>(path: P) -> Result<()> {
    if path.as_ref().is_file() {
        Ok(try!(fs::remove_file(&path)
                .chain_err(|| format!("Unable to delete file {}",
                                      path.as_ref().display()))))
    } else {
        Ok(())
    }
}

pub fn remove_dir_if_exists<P: AsRef<Path>>(path: P) -> Result<()> {
    if path.as_ref().is_dir() {
        Ok(try!(fs::remove_dir_all(&path)
                .chain_err(|| format!("Unable to delete directory {}",
                                      path.as_ref().display()))))
    } else {
        Ok(())
    }
}
