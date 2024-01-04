use std::env;

mod command;
mod object;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    if command == "init" {
        command::init::init();
        return;
    }

    if let Err(e) = util::path::find_git_root() {
        println!("{}", e);
        return;
    }

    match command.as_str() {
        "add" => {
            let file_names = &args[2..];
            command::add::add(file_names);
        }
        "commit" => match &args[2].to_string()[..] {
            "-m" => {
                let message = &args[3].to_string();
                command::commit::commit(message.to_string());
            }
            _ => {
                println!("no commit message");
                return;
            }
        },
        "branch" if &args[2] != "-d" => {
            let branch_name = &args[2].to_string();
            command::branch::branch(branch_name.to_string());
        }
        "branch" => {
            let branch_name = &args[3].to_string();
            command::branch::delete_branch(branch_name.to_string());
        }
        "checkout" => {
            let branch_name = &args[2].to_string();
            command::branch::checkout(branch_name.to_string());
        }
        "log" => {
            //TODO: impl
        }
        _ => (),
    }
}
