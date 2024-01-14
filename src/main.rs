use std::env;

mod command;
mod object;
mod util;

#[derive(Debug, Clone)]
enum Command {
    Init,
    Add {
        files: Vec<String>, /* it's better to use PathBuf here! */
    },
    Commit {
        message: String,
    },
    Branch {
        delete: bool,
        name: String,
    },
    Checkout {
        branch: String,
    },
    Log,
}

fn options() -> Command {
    use bpaf::*;

    let init = pure(Command::Init)
        .to_options()
        .command("init")
        .help("Initialize new git repository");

    let add = {
        let files = positional("FILE").help("File to add").many();
        construct!(Command::Add { files })
            .to_options()
            .command("add")
            .help("Add a file")
    };

    let commit = {
        let message = short('m')
            .long("message")
            .help("Commit changes")
            .argument("MESSAGE");
        construct!(Command::Commit { message })
            .to_options()
            .command("commit")
            .help("Commit changes")
    };

    let branch = {
        let delete = short('d')
            .long("delete")
            .help("Delete the branch?")
            .switch();
        let name = positional("BRANCH").help("Branch name");
        construct!(Command::Branch { delete, name })
            .to_options()
            .command("branch")
    };

    let checkout = {
        let branch = positional("BRANCH").help("Branch name");
        construct!(Command::Checkout { branch })
            .to_options()
            .command("checkout")
    };

    let log = pure(Command::Log).to_options().command("log");

    construct!([init, add, commit, branch, checkout, log])
        .to_options()
        .version(env!("CARGO_PKG_VERSION"))
        .fallback_to_usage()
        .run()
}

fn main() {
    let args = options();
    println!("Args: {args:?}");

    if !matches!(args, Command::Init) {
        if let Err(e) = util::path::find_git_root() {
            println!("{}", e);
            return;
        }
    }

    match args {
        Command::Init => command::init::init(),
        Command::Add { files } => command::add::add(&files),
        Command::Commit { message } => command::commit::commit(message),
        Command::Branch { delete, name } => {
            if delete {
                command::branch::delete_branch(name);
            } else {
                command::branch::branch(name);
            }
        }
        Command::Checkout { branch } => {
            command::branch::checkout(branch);
        }
        Command::Log => todo!(),
    }
}
