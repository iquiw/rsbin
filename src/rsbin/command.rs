use std::env::Args;
use std::fmt;

use failure::{err_msg, Error};

use rsbin::config::{RsbinConfig, RsbinScript};
use rsbin::os::RsbinEnv;
use rsbin::util;

pub fn clean(env: &RsbinEnv, cfg: &RsbinConfig) -> Result<(), Error> {
    for scr in &cfg.scripts {
        try!(util::remove_file_if_exists(env.bin_path(scr)));
        try!(util::remove_file_if_exists(env.hash_path(scr)));
        try!(util::remove_dir_if_exists(env.tmp_path(scr)));
    }
    Ok(())
}

pub fn help() -> Result<(), Error> {
    println!(
        "usage: rsbin COMMAND [ARG..]

commands:
  help                        : show this help
  clean                       : clean garbages
  list [-l]                   : list available scripts (ls)
  run NAME                    : run script NAME, build it if necessary
  update [-f] [NAME..]        : compile all or specified scripts if necessary
"
    );
    Ok(())
}

pub fn list(cfg: &RsbinConfig, args: &mut Args) -> Result<(), Error> {
    let long = if let Some(ref s) = args.next() {
        s == "-l"
    } else {
        false
    };
    println!("Available scripts:");
    for scr in &cfg.scripts {
        if long {
            println!("  {:12} {}", scr.name, scr.path);
        } else {
            println!("  {}", scr.name);
        }
    }
    Ok(())
}

pub fn run(env: &RsbinEnv, cfg: &RsbinConfig, args: &mut Args) -> Result<(), Error> {
    match args.next() {
        Some(name) => match lookup_script(cfg, &name) {
            Some(scr) => {
                let scr_args: Vec<_> = args.collect();
                try!(update_script(env, scr, false));
                scr.execute(env, &scr_args)
            }
            None => Err(err_msg("script not found")),
        },
        None => Err(err_msg("run needs script name")),
    }
}

pub fn update(env: &RsbinEnv, cfg: &RsbinConfig, args: &mut Args) -> Result<(), Error> {
    let mut args = args.peekable();
    let force = if Some("-f") == args.peek().map(|s| s.as_ref()) {
        args.next();
        true
    } else {
        false
    };
    if args.peek().is_none() {
        for scr in &cfg.scripts {
            let res = try!(update_script(env, scr, force));
            println!("{:12} {}", res, scr.name);
        }
    } else {
        for name in args {
            let res = match lookup_script(cfg, &name) {
                Some(scr) => try!(update_script(env, scr, force)),
                None => RsbinUpdateResult::NotFound,
            };
            println!("{:12} {}", res, name);
        }
    }
    Ok(())
}

enum RsbinUpdateResult {
    Latest,
    Compiled,
    NotFound,
}

impl fmt::Display for RsbinUpdateResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RsbinUpdateResult::Latest => f.pad("[LATEST]"),
            RsbinUpdateResult::Compiled => f.pad("[COMPILED]"),
            RsbinUpdateResult::NotFound => f.pad("[NOT FOUND]"),
        }
    }
}

fn update_script(
    env: &RsbinEnv,
    scr: &RsbinScript,
    force: bool,
) -> Result<RsbinUpdateResult, Error> {
    let hash = try!(scr.get_hash());
    if force || !try!(scr.is_hash_same(env, &hash)) || !scr.does_bin_exist(env) {
        try!(scr.compile(env));
        try!(scr.write_hash(env, &hash));
        Ok(RsbinUpdateResult::Compiled)
    } else {
        Ok(RsbinUpdateResult::Latest)
    }
}

fn lookup_script<'a>(cfg: &'a RsbinConfig, name: &str) -> Option<&'a RsbinScript> {
    for scr in &cfg.scripts {
        if scr.name == name {
            return Some(scr);
        }
    }
    None
}
