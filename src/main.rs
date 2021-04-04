#![warn(clippy::all, clippy::pedantic)]
use getopts::Options;

use std::{env, process};
use std::path::PathBuf;

mod command;
mod config;
mod gui;
mod keys;

use config::Config;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CONFIG: Config = Config::get();
    static ref CONFIGDIR: PathBuf = config::get_config_dir();
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} URI [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let progname = args[0].split('/').last().unwrap_or("rwb");
    let usage = format!("Usage: {} uri", progname);
    let mut opts = Options::new();
    opts.optflag("p", "private", "Private browsing");
    opts.optflag("h", "help", "Print this help message");
    let args = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(m) => {
            eprintln!("Error: {}", m.to_string());
            eprintln!("{}", usage);
            process::exit(1);
        }
    };
    if args.opt_present("h") {
        print_usage(&progname, opts);
        return;
    }
    gui::run(args);
}
