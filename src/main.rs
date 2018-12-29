extern crate crypto;
extern crate failure;
extern crate serde;
extern crate toml;

mod rsbin;

use failure::Error;

use rsbin::command;
use rsbin::config::RsbinConfig;
use rsbin::os::RsbinEnv;

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
        Some(cmd) => match cmd.as_ref() {
            "clean" => command::clean(env, cfg),
            "list" | "ls" => command::list(cfg, &mut args),
            "run" => command::run(env, cfg, &mut args),
            "update" => command::update(env, cfg, &mut args),
            "help" | _ => command::help(),
        },
        _ => command::help(),
    }
}
