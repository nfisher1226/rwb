use crate::CONFIG;
use crate::url::Url;

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
        "http" => url.to_string(),
        "https" => url.to_string(),
        "file" => url.to_string(),
        "ftp" => url.to_string(),
        "about" => url.to_string(),
        _ => format!("http://{}", url),
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
            2 => {
                match CONFIG.quickmarks.get(cmd_string[1]) {
                    Some(c) => String::from(c),
                    None => match Url::parse(cmd_string[1]) {
                        Ok(_) => String::from(cmd_string[1]),
                        Err(_) => Command::search_default(vec!(cmd_string[1])),
                    }
                }
            },
            n if n > 2 => {
                match CONFIG.searchengines.get(cmd_string[1]) {
                    Some(c) => {
                        let search = &cmd_string[2..];
                        Command::search_custom(String::from(c), search.to_vec())
                    },
                    None => {
                        let search = &cmd_string[1..];
                        Command::search_default(search.to_vec())
                    },
                }
            },
            _ => String::from(cmd_string[1])
        };
        Command {
            command,
            uri,
        }
    }

    pub fn search_default(search: Vec<&str>) -> String {
        let engine = match CONFIG.global.get("default_search") {
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
