#![warn(clippy::all, clippy::pedantic)]
use serde::Deserialize;
use xdg_basedir::*;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::{env, process};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub homepage: Option<String>,
    pub default_search: Option<String>,
    pub global: HashMap<String, String>,
    pub quickmarks: HashMap<String, String>,
    pub searchengines: HashMap<String, String>,
}

pub fn get_config_dir() -> PathBuf {
    let mut configdir: PathBuf = match get_config_home() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    let progname = env!("CARGO_PKG_NAME");
    configdir.push(progname);
    configdir
}

impl Config {
    pub fn get() -> Config {
        let mut config: PathBuf = get_config_dir();
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
