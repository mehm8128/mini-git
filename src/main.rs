use std::env;

mod command;
mod util;
mod object;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    if command == "init" {
        command::init::init();
        return;
    }

    if let Err(e) = util::path::find_git_root(".".to_string()) {
        println!("{}", e);
        return;
    }

    match command.as_str() {
        "add" => {
            let file_names = &args[2..];
            command::add::add(file_names);
        }
        "commit" => {
            let file_names = &args[2..];
            let message = "commit message";
            command::commit::commit(file_names, message.to_string());
        }
        "log" => {
            //TODO: impl
        }
        _ => (),
    }
}
