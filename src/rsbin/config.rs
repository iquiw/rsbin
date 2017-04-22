use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::result;

use serde::de::Error as SerdeError;
use serde::de::{Deserialize, Deserializer, IntoDeserializer};
use toml;

use rsbin::errors::*;

#[derive(Debug, Deserialize)]
pub enum RsbinBuildType {
    #[serde(rename="rustc")] Rustc,
    #[serde(rename="cargo")] Cargo,
    #[serde(rename="ghc")]   Ghc,
    #[serde(rename="stack")] Stack,
}

#[derive(Debug, Deserialize)]
pub struct RsbinScript {
    pub name: String,
    pub path: String,
    #[serde(rename="build-type", deserialize_with="deserialize_build_type")]
    pub build_type: RsbinBuildType,
    #[serde(rename="build-opts", default)]
    pub build_opts: Vec<String>,
    #[serde(rename="build-deps", default)]
    build_deps: Vec<String>,
}

fn deserialize_build_type<'de, D>(d: D) -> result::Result<RsbinBuildType, D::Error> where D: Deserializer<'de> {
    let toml: toml::Value = try!(Deserialize::deserialize(d));
    match toml {
        toml::Value::String(s) => {
            let sd = s.to_lowercase().into_deserializer();
            Ok(try!(Deserialize::deserialize(sd)))
        },
        _ => Err(D::Error::custom(format!("invalid type: {}", toml.type_str())))
    }
}

#[derive(Debug, Deserialize)]
pub struct RsbinConfig {
    pub scripts: Vec<RsbinScript>
}


impl RsbinConfig {
    pub fn load<P>(path: P) -> Result<RsbinConfig> where P: AsRef<Path> {
        let mut f = try!(File::open(&path)
                         .chain_err(|| format!("Unable to open {}", path.as_ref().display())));
        let mut s = String::new();
        try!(f.read_to_string(&mut s).chain_err(|| "read_to_string"));

        toml::from_str(&s).chain_err(|| "Invalid TOML format")
    }
}
