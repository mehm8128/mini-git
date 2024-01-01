use std::env;

mod command;
mod util;

struct Client {
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
    println!("{}", client.root_dir);

    match command.as_str() {
        "add" => {
            //TODO:impl
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
