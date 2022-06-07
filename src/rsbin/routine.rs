use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, Context, Result};

use super::config::{RsbinBuildType, RsbinScript};
use super::os::RsbinEnv;
use super::util::create_dir_if_missing;

impl RsbinScript {
    pub fn execute<S>(&self, env: &RsbinEnv, args: &[S]) -> Result<()>
    where
        S: AsRef<OsStr>,
    {
        let path = env.bin_path(self);
        run_command(&path, Command::new(&path).args(args))
    }

    pub fn compile(&self, env: &RsbinEnv) -> Result<()> {
        match self.build_type {
            RsbinBuildType::Rustc => build_rustc(Path::new(&self.path), &env.bin_path(self)),
            RsbinBuildType::Ghc => build_ghc(
                Path::new(&self.path),
                &env.bin_path(self),
                &env.tmp_path(self),
                &self.build_opts,
            ),
            _ => Err(anyhow!("Unsupported build-type")),
        }
    }

    pub fn does_bin_exist(&self, env: &RsbinEnv) -> bool {
        env.bin_path(self).is_file()
    }
}

fn run_command(path: &Path, cmd: &mut Command) -> Result<()> {
    let status = cmd
        .status()
        .with_context(|| format!("{}: execution failed", path.display()))?;
    if status.success() {
        Ok(())
    } else {
        match status.code() {
            Some(code) => Err(anyhow!("{}: process exited with {}", path.display(), code)),
            None => Err(anyhow!("{}: interrupted by signal", path.display())),
        }
    }
}

fn build_rustc(src: &Path, dst: &Path) -> Result<()> {
    let path = Path::new("rustc");
    run_command(path, Command::new(path).arg("-o").arg(dst).arg(src))
}

fn build_ghc(src: &Path, dst: &Path, tmpdir: &Path, opts: &[String]) -> Result<()> {
    create_dir_if_missing(tmpdir)?;
    let path = Path::new("ghc");
    run_command(
        path,
        Command::new(path)
            .args(opts)
            .arg("-outputdir")
            .arg(tmpdir)
            .arg("-o")
            .arg(dst)
            .arg(src),
    )
}
