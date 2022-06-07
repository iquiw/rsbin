mod rsbin;

use anyhow::Result;

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

fn dispatch(env: &RsbinEnv, cfg: &RsbinConfig) -> Result<()> {
    let mut args = std::env::args();

    args.next();
    match args.next() {
        Some(cmd) => match cmd.as_ref() {
            "clean" => command::clean(env, cfg),
            "list" | "ls" => command::list(cfg, &mut args),
            "run" => command::run(env, cfg, &mut args),
            "update" => command::update(env, cfg, &mut args),
            _ => command::help(),
        },
        _ => command::help(),
    }
}
