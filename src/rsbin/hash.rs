use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::fs::File;
use std::path::Path;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

use rsbin::os::RsbinEnv;
use rsbin::config::RsbinScript;
use rsbin::errors::{Result, ResultExt};

impl RsbinScript {
    pub fn get_hash(&self) -> Result<String> {
        Ok(try!(hash_file(&self.path)
            .chain_err(|| format!("Unable to calculate hash of {}", &self.path))))
    }

    pub fn write_hash(&self, env: &RsbinEnv, hash: &str) -> Result<()> {
        let path = env.hash_path(self);
        let mut f = try!(File::create(&path)
            .chain_err(|| format!("Unable to create {}", path.display())));
        try!(f.write_fmt(format_args!("{}", hash))
            .chain_err(|| "Unable to write hash"));
        Ok(())
    }

    pub fn is_hash_same(&self, env: &RsbinEnv, hash: &str) -> Result<bool> {
        let path = env.hash_path(self);
        if let Ok(mut f) = File::open(path) {
            let mut s = String::new();
            try!(f.read_to_string(&mut s)
                .chain_err(|| "Unable to read hash"));
            Ok(s == hash)
        } else {
            Ok(false)
        }
    }
}

fn hash_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
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
