#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
extern crate crypto;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod rsbin;

use std::io::Write;
use std::error::Error;

use rsbin::os::RsbinEnv;
use rsbin::config::RsbinConfig;
use rsbin::command;
use rsbin::errors::Result;

fn main() {
    let env = RsbinEnv::new();

    let result = env.init().and_then(|cfg| dispatch(&env, &cfg));
    if let Err(err) = result {
        let mut msg = String::from(err.description());
        if let Some(cause) = err.cause() {
            msg.push_str(format!(", {}", cause).as_ref());
        }
        let _ = writeln!(std::io::stderr(), "{}", msg);
    }
}

fn dispatch(env: &RsbinEnv, cfg: &RsbinConfig) -> Result<()> {
    let mut args = std::env::args();

    args.next();
    match args.next() {
        Some(cmd) => {
            match cmd.as_ref() {
                "clean" => command::clean(env, cfg),
                "list" | "ls" => command::list(cfg, &mut args),
                "run" => command::run(env, cfg, &mut args),
                "update" => command::update(env, cfg, &mut args),
                "help" | _ => command::help(),
            }
        }
        _ => command::help(),
    }
}
