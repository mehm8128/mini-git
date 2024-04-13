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
        name: String,
        delete: bool,
    },
    Checkout {
        name: String,
        new_branch: bool,
    },
    Log,
}

fn options() -> Command {
    use bpaf::{construct, positional, pure, short, Parser};

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
        let new_branch = short('b').help("Create new branch?").switch();
        let name = positional("BRANCH").help("Branch name");
        construct!(Command::Checkout { name, new_branch })
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

fn main() -> anyhow::Result<()> {
    let args = options();
    println!("Args: {args:?}");

    if !matches!(args, Command::Init) {
        util::path::find_git_root()?;
    }

    match args {
        Command::Init => command::init::init()?,
        Command::Add { files } => command::add::add(&files)?,
        Command::Commit { message } => command::commit::commit(message)?,
        Command::Branch { name, delete } => {
            if delete {
                command::branch::delete(&name)?;
            } else {
                // show branches
                todo!();
            }
        }
        Command::Checkout { name, new_branch } => {
            if new_branch {
                command::branch::create(&name)?;
            } else {
                command::branch::checkout(&name)?;
            }
        }
        Command::Log => todo!(),
    };
    Ok(())
}
