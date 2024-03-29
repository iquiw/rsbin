use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::{Error, Result};

use serde::de;
use serde::de::Error as SerdeError;
use serde::de::{Deserializer, IntoDeserializer};
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum RsbinBuildType {
    #[serde(rename = "rustc")]
    Rustc,
    #[serde(rename = "cargo")]
    Cargo,
    #[serde(rename = "ghc")]
    Ghc,
    #[serde(rename = "stack")]
    Stack,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct RsbinScript {
    pub name: String,
    pub path: String,
    #[serde(rename = "build-type", deserialize_with = "deserialize_build_type")]
    pub build_type: RsbinBuildType,
    #[serde(rename = "build-opts", default)]
    pub build_opts: Vec<String>,
    #[serde(rename = "build-deps", default)]
    build_deps: Vec<String>,
}

fn deserialize_build_type<'de, D>(d: D) -> Result<RsbinBuildType, D::Error>
where
    D: Deserializer<'de>,
{
    let toml: toml::Value = de::Deserialize::deserialize(d)?;
    match toml {
        toml::Value::String(s) => {
            let sd = s.to_lowercase().into_deserializer();
            Ok(de::Deserialize::deserialize(sd)?)
        }
        _ => Err(D::Error::custom(format!(
            "invalid type: {}",
            toml.type_str()
        ))),
    }
}

#[derive(Debug, Deserialize)]
pub struct RsbinConfig {
    pub scripts: Vec<RsbinScript>,
}

impl RsbinConfig {
    pub fn load<P>(path: P) -> Result<RsbinConfig>
    where
        P: AsRef<Path>,
    {
        let mut f = File::open(&path).map_err(|e| {
            let ctx = format!("Unable to open {}, ERROR: {}", path.as_ref().display(), e);
            Error::new(e).context(ctx)
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;

        toml::from_str(&s).map_err(|e| {
            let ctx = format!("Invalid TOM format, ERROR: {}", e);
            Error::new(e).context(ctx)
        })
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
