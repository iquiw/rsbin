use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

use sha1::{Digest, Sha1};

use anyhow::{Context, Result};

use super::config::RsbinScript;
use super::os::RsbinEnv;

impl RsbinScript {
    pub fn get_hash(&self) -> Result<String> {
        hash_file(&self.path).with_context(|| format!("Unable to calculate hash of {}", &self.path))
    }

    pub fn write_hash(&self, env: &RsbinEnv, hash: &str) -> Result<()> {
        let path = env.hash_path(self);
        let mut f =
            File::create(&path).with_context(|| format!("Unable to create {}", path.display()))?;
        f.write_fmt(format_args!("{}", hash))
            .with_context(|| "Unable to write hash")?;
        Ok(())
    }

    pub fn is_hash_same(&self, env: &RsbinEnv, hash: &str) -> Result<bool> {
        let path = env.hash_path(self);
        if let Ok(mut f) = File::open(path) {
            let mut s = String::new();
            f.read_to_string(&mut s)
                .with_context(|| "Unable to read hash")?;
            Ok(s == hash)
        } else {
            Ok(false)
        }
    }
}

fn hash_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha1::new();
    loop {
        let len = {
            let buf = reader.fill_buf()?;
            hasher.update(buf);
            buf.len()
        };
        if len == 0 {
            break;
        }
        reader.consume(len);
    }
    Ok(format!("{:x}", hasher.finalize()))
}
