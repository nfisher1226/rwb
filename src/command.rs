#![warn(clippy::all, clippy::pedantic)]
use crate::url::Url;
use crate::CONFIG;

pub struct Command {
    pub command: String,
    pub uri: String,
}

pub fn parse_url(url: &str) -> String {
    let url_split = url.split(':').collect::<Vec<&str>>();
    if url_split.is_empty() {
        return String::from("about:blank");
    }
    match url_split[0] {
        "http" | "https" | "file" | "ftp" | "about" => url.to_string(),
        __ => {
            if let Some(_) = url.find('.') {
                format!("http://{}", url)
            } else {
                let url: Vec<&str> = vec![url];
                Command::search_default(url)
            }
        }
    }
}

impl Command {
    pub fn new(cmd_string: String) -> Command {
        let cmd_string: Vec<&str> = cmd_string.split_whitespace().collect();
        let command = String::from(cmd_string[0]);
        if command.as_str() == ":quit" {
            gtk::main_quit();
        }
        let uri = match cmd_string.len() {
            1 => String::from("about:blank"),
            2 => match CONFIG.quickmarks.get(cmd_string[1]) {
                Some(c) => String::from(c),
                None => {
                    let url_test = parse_url(cmd_string[1]);
                    match Url::parse(&url_test) {
                        Ok(_) => url_test,
                        Err(_) => Command::search_default(vec![cmd_string[1]]),
                    }
                }
            },
            n if n > 2 => match CONFIG.searchengines.get(cmd_string[1]) {
                Some(c) => {
                    let search = &cmd_string[2..];
                    Command::search_custom(String::from(c), search.to_vec())
                }
                None => {
                    let search = &cmd_string[1..];
                    Command::search_default(search.to_vec())
                }
            },
            _ => String::from(cmd_string[1]),
        };
        Command { command, uri }
    }

    pub fn search_default(search: Vec<&str>) -> String {
        let engine = match &CONFIG.default_search {
            Some(c) => String::from(c),
            None => String::from("https://duckduckgo.com/?q={}&ia=web"),
        };
        let search = search.join("+");
        engine.replace("{}", &search)
    }

    pub fn search_custom(engine: String, search: Vec<&str>) -> String {
        let search = search.join("+");
        engine.replace("{}", &search)
    }
}
