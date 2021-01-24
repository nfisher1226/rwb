use getopts::Options;
extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate url;
extern crate webkit2gtk;

use std::{env, process};

mod command;
mod config;
mod gui;
mod keys;

use config::Config;

#[macro_use]
extern crate lazy_static;

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
