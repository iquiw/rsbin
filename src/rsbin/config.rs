use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::result;

use serde::de::Error as SerdeError;
use serde::de::{Deserialize, Deserializer, IntoDeserializer};
use toml;

use rsbin::errors::*;

#[derive(Debug, Deserialize, PartialEq)]
pub enum RsbinBuildType {
    #[serde(rename="rustc")] Rustc,
    #[serde(rename="cargo")] Cargo,
    #[serde(rename="ghc")]   Ghc,
    #[serde(rename="stack")] Stack,
}

#[derive(Debug, Deserialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_from_str_without_optional() {
        let s = r#"
name = "foo"
path = "/bar/foo.rs"
build-type = "rustc"
"#;
        let rs_exp = RsbinScript {
            name: "foo".to_owned(),
            path: "/bar/foo.rs".to_owned(),
            build_type: RsbinBuildType::Rustc,
            build_opts: vec![],
            build_deps: vec![],
        };
        assert_eq!(toml::from_str::<RsbinScript>(&s).unwrap(), rs_exp);
    }

    #[test]
    fn test_script_from_str_with_build_opts() {
        let s = r#"
name = "foo"
path = "/bar/foo.hs"
build-type = "ghc"
build-opts = ["-O2"]
"#;
        let rs_exp = RsbinScript {
            name: "foo".to_owned(),
            path: "/bar/foo.hs".to_owned(),
            build_type: RsbinBuildType::Ghc,
            build_opts: vec!["-O2".to_owned()],
            build_deps: vec![],
        };
        assert_eq!(toml::from_str::<RsbinScript>(&s).unwrap(), rs_exp);
    }

    #[test]
    fn test_script_from_str_with_build_deps() {
        let s = r#"
name = "foo"
path = "/bar/foo.rs"
build-type = "cargo"
build-deps = ["serde = \"^1.0.0\"", "toml"]
"#;
        let rs_exp = RsbinScript {
            name: "foo".to_owned(),
            path: "/bar/foo.rs".to_owned(),
            build_type: RsbinBuildType::Cargo,
            build_opts: vec![],
            build_deps: vec!["serde = \"^1.0.0\"".to_owned(), "toml".to_owned()],
        };
        assert_eq!(toml::from_str::<RsbinScript>(&s).unwrap(), rs_exp);
    }
}
