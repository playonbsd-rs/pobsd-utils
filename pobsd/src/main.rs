use clap::{arg, Command};
use pledge::pledge_promises;
use pobsd::{browse, check, export};
use unveil::unveil;

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
            Command::new("browse")
                .about("Browse the game in the Database")
                .arg(arg!(<DATABASE> "The Database"))
                .arg_required_else_help(true),
        )
}

fn main() -> Result<(), std::io::Error> {
    pledge_promises![Stdio Unveil Rpath Wpath Cpath Tty]
        .or_else(pledge::Error::ignore_platform)
        .unwrap();
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("check", sub_matches)) => {
            pledge_promises![Stdio Unveil Rpath]
                .or_else(pledge::Error::ignore_platform)
                .unwrap();
            let db = sub_matches.get_one::<String>("DATABASE").expect("required");
            unveil(db, "r")
                .or_else(unveil::Error::ignore_platform)
                .unwrap();
            unveil("", "")
                .or_else(unveil::Error::ignore_platform)
                .unwrap();
            check(db)?
        }
        Some(("export", sub_matches)) => {
            pledge_promises![Stdio Unveil Rpath Wpath Cpath]
                .or_else(pledge::Error::ignore_platform)
                .unwrap();
            let db = sub_matches.get_one::<String>("DATABASE").expect("required");
            let js = sub_matches.get_one::<String>("JSON").expect("required");
            unveil(db, "r")
                .or_else(unveil::Error::ignore_platform)
                .unwrap();
            unveil(js, "cw")
                .or_else(unveil::Error::ignore_platform)
                .unwrap();
            unveil("", "")
                .or_else(unveil::Error::ignore_platform)
                .unwrap();
            export(db, js)?
        }
        Some(("browse", sub_matches)) => {
            pledge_promises![Stdio Unveil Rpath Tty]
                .or_else(pledge::Error::ignore_platform)
                .unwrap();
            let db = sub_matches.get_one::<String>("DATABASE").expect("required");
            unveil(db, "r")
                .or_else(unveil::Error::ignore_platform)
                .unwrap();
            unveil("", "")
                .or_else(unveil::Error::ignore_platform)
                .unwrap();
            browse(db)?
        }
        _ => println!("Unsupported command"),
    }
    Ok(())
}
