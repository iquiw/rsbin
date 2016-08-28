use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use rsbin::errors::{ChainErr, Result};
use rsbin::os::RsbinEnv;
use rsbin::config::{RsbinBuildType, RsbinScript};
use rsbin::util::{create_dir_if_missing, err};

impl RsbinScript {
    pub fn execute<S>(&self, env: &RsbinEnv, args: &[S]) -> Result<()>
        where S: AsRef<OsStr> {
        let path = env.bin_path(self);
        run_command(&path, Command::new(&path).args(args))
    }

    pub fn compile(&self, env: &RsbinEnv) -> Result<()> {
        match self.build_type {
            RsbinBuildType::Rustc =>
                build_rustc(Path::new(&self.path), &env.bin_path(self)),
            RsbinBuildType::Ghc =>
                build_ghc(Path::new(&self.path), &env.bin_path(self),
                          &env.tmp_path(self), &self.build_opts),
            _ => err("Unsupported build-type")
        }
    }

    pub fn does_bin_exist(&self, env: &RsbinEnv) -> bool {
        env.bin_path(self).is_file()
    }
}

fn run_command(path: &Path, cmd: &mut Command) -> Result<()> {
    let status = try!(cmd.status()
                      .chain_err(|| format!("{}: execution failed",
                                            path.display())));
    if status.success() {
        Ok(())
    } else {
        match status.code() {
            Some(code) => err(format!("{}: process exited with {}",
                                      path.display(), code)),
            None => err(format!("{}: interrupted by signal", path.display()))
        }
    }
}

fn build_rustc(src: &Path, dst: &Path) -> Result<()> {
    let path = Path::new("rustc");
    run_command(path, Command::new(path).arg("-o").arg(dst).arg(src))
}

fn build_ghc(src: &Path, dst: &Path, tmpdir: &Path, opts: &[String])
             -> Result<()> {
    try!(create_dir_if_missing(tmpdir));
    let path = Path::new("ghc");
    run_command(
        path,
        Command::new(path).args(opts)
            .arg("-outputdir").arg(tmpdir)
            .arg("-o").arg(dst).arg(src))
}
