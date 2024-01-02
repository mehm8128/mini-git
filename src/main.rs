use std::env;

mod command;
mod util;

pub struct Client {
    root_dir: String,
}

impl Client {
    fn new(path: String) -> Client {
        let root_dir = util::path::find_git_root(path).unwrap();
        Client { root_dir }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    if command == "init" {
        command::init::init();
        return;
    }

    let client = Client::new(".".to_string());

    match command.as_str() {
        "add" => {
            let file_names = &args[2..];
            command::add::add(client, file_names);
        }
        "commit" => {
            //TODO:impl
        }
        "log" => {
            //TODO: impl
        }
        _ => (),
    }
}
