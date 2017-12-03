use std::io::{BufRead, BufReader, Read, Write};
use std::fs::File;
use std::path::Path;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

use failure::{Error, ResultExt};

use rsbin::os::RsbinEnv;
use rsbin::config::RsbinScript;

impl RsbinScript {
    pub fn get_hash(&self) -> Result<String, Error> {
        Ok(hash_file(&self.path)
           .with_context(|_| format!("Unable to calculate hash of {}", &self.path))?)
    }

    pub fn write_hash(&self, env: &RsbinEnv, hash: &str) -> Result<(), Error> {
        let path = env.hash_path(self);
        let mut f = File::create(&path)
            .with_context(|_| format!("Unable to create {}", path.display()))?;
        f.write_fmt(format_args!("{}", hash))
            .with_context(|_| "Unable to write hash")?;
        Ok(())
    }

    pub fn is_hash_same(&self, env: &RsbinEnv, hash: &str) -> Result<bool, Error> {
        let path = env.hash_path(self);
        if let Ok(mut f) = File::open(path) {
            let mut s = String::new();
            f.read_to_string(&mut s).with_context(|_| "Unable to read hash")?;
            Ok(s == hash)
        } else {
            Ok(false)
        }
    }
}

fn hash_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let file = try!(File::open(&path));
    let mut reader = BufReader::new(file);
    let mut hasher = Sha1::new();
    loop {
        let len = {
            let buf = try!(reader.fill_buf());
            hasher.input(buf);
            buf.len()
        };
        if len == 0 {
            break;
        }
        reader.consume(len);
    }
    Ok(hasher.result_str())
}
