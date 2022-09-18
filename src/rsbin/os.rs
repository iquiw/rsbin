use std::env;
use std::path::PathBuf;

use anyhow::Result;

use super::config::{RsbinConfig, RsbinScript};
use super::util;

pub struct RsbinEnv {
    appdir: PathBuf,
    tmpdir: PathBuf,
}

impl RsbinEnv {
    pub fn new() -> RsbinEnv {
        RsbinEnv {
            appdir: appdir(),
            tmpdir: tmpdir(),
        }
    }

    pub fn init(&self) -> Result<RsbinConfig> {
        util::create_dir_if_missing(&self.appdir)?;
        util::create_dir_if_missing(&self.tmpdir)?;
        util::create_dir_if_missing(&self.bindir())?;
        util::create_dir_if_missing(&self.hashdir())?;
        RsbinConfig::load(self.config_path())
    }

    fn bindir(&self) -> PathBuf {
        let mut bindir = PathBuf::from(&self.appdir);
        bindir.push("bin");
        bindir
    }

    pub fn bin_path(&self, scr: &RsbinScript) -> PathBuf {
        let mut binpath = self.bindir();
        binpath.push(&scr.name);
        binpath.set_extension(std::env::consts::EXE_EXTENSION);
        binpath
    }

    pub fn config_path(&self) -> PathBuf {
        let mut cfgpath = PathBuf::from(&self.appdir);
        cfgpath.push("config.toml");
        cfgpath
    }

    fn hashdir(&self) -> PathBuf {
        let mut hashdir = PathBuf::from(&self.appdir);
        hashdir.push("hash");
        hashdir
    }

    pub fn hash_path(&self, scr: &RsbinScript) -> PathBuf {
        let mut hashpath = self.hashdir();
        hashpath.push(&scr.name);
        hashpath
    }

    pub fn tmp_path(&self, scr: &RsbinScript) -> PathBuf {
        let mut tmppath = tmpdir();
        tmppath.push(&scr.name);
        tmppath
    }
}

#[cfg(unix)]
fn appdir() -> PathBuf {
    let home = env::var("HOME").expect("HOME is set");
    let mut appdir = PathBuf::from(home);
    appdir.push(".rsbin");
    appdir
}
#[cfg(unix)]
fn tmpdir() -> PathBuf {
    let tmp = env::var("TMP").unwrap_or("/tmp".to_string());
    let mut tmpdir = PathBuf::from(tmp);
    tmpdir.push("rsbin");
    tmpdir
}
#[cfg(windows)]
fn appdir() -> PathBuf {
    let appdata = env::var("APPDATA").expect("APPDATA is set");
    let mut appdir = PathBuf::from(appdata);
    appdir.push("rsbin");
    appdir
}
#[cfg(windows)]
fn tmpdir() -> PathBuf {
    let tmp = env::var("TMP")
        .or_else(|_| env::var("TEMP"))
        .expect("TMP is set");
    let mut tmpdir = PathBuf::from(tmp);
    tmpdir.push("rsbin");
    tmpdir
}
