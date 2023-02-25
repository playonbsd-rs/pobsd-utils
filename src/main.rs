use clap::{arg, Command};

use pobsd_rs::{add_game, browse, check, export};

fn cli() -> Command {
    Command::new("pobsdrs-parser")
        .about("A tool to interact and manipulate the PlayOnBSD Database")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("check")
                .about("Check for error in the Database")
                .arg(arg!(<DATABASE> "The Database"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("export")
                .about("Export the Database in json format")
                .arg(arg!(<DATABASE> "The Database"))
                .arg_required_else_help(true)
                .arg(arg!(<JSON> "The json file"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("add")
                .about("Add a game in the Database")
                .arg(arg!(<DATABASE> "The Database"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("browse")
                .about("Browse the game in the Database")
                .arg(arg!(<DATABASE> "The Database"))
                .arg_required_else_help(true),
        )
}

fn main() -> Result<(), std::io::Error> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("check", sub_matches)) => {
            let db = sub_matches.get_one::<String>("DATABASE").expect("required");
            check(db)?
        }
        Some(("export", sub_matches)) => {
            let db = sub_matches.get_one::<String>("DATABASE").expect("required");
            let js = sub_matches.get_one::<String>("JSON").expect("required");
            export(db, js)?
        }
        Some(("add", sub_matches)) => {
            let db = sub_matches.get_one::<String>("DATABASE").expect("required");
            add_game(db)
            //add_game(file)?
        }
        Some(("browse", sub_matches)) => {
            let db = sub_matches.get_one::<String>("DATABASE").expect("required");
            browse(db)?
        }
        _ => println!("Unsupported command"),
    }
    Ok(())
}
