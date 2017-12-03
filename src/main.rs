extern crate failure;
extern crate crypto;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod rsbin;

use failure::Error;

use rsbin::os::RsbinEnv;
use rsbin::config::RsbinConfig;
use rsbin::command;

fn main() {
    let env = RsbinEnv::new();

    let result = env.init().and_then(|cfg| dispatch(&env, &cfg));
    if let Err(err) = result {
        eprintln!("{}", err);
    }
}

fn dispatch(env: &RsbinEnv, cfg: &RsbinConfig) -> Result<(), Error> {
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
