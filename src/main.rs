use getopts::Options;
use serde::Deserialize;
use xdg_basedir::*;
extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate url;
extern crate webkit2gtk;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::{env, process};

mod gui;
mod keys;

#[macro_use]
extern crate lazy_static;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub global: HashMap<String, String>,
    pub quickmarks: HashMap<String, String>,
    pub searchengines: HashMap<String, String>
}

impl Config {
    fn get() -> Config {
        let mut config: PathBuf = match get_config_home() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
        let progname = env!("CARGO_PKG_NAME");
        config.push(progname);
        config.push("config.toml");
        let config = if config.exists() {
            match fs::read_to_string(config) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }
        } else {
            include_str!("config.toml").to_string()
        };
        let config: Config = match toml::from_str(&config) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        };
        config
    }
}

lazy_static! {
    static ref CONFIG: Config = {
        Config::get()
    };
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let progname = args[0].split('/').last().unwrap();
    let usage = format!("Usage: {} uri", progname);
    let opts = Options::new();
    let args = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(m) => {
            eprintln!("Error: {}", m.to_string());
            eprintln!("{}", usage);
            process::exit(1);
        }
    };
    let uri = if args.free.len() == 1 {
        &args.free[0]
    } else {
        match CONFIG.global.get("homepage") {
            Some(c) => c,
            None => "https://duckduckgo.com"
        }
    };
    gui::run(uri);
}
