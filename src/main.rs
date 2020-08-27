use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Gitfindr-rs")
        .about("Helps you manage your local git repositories.")
        .version("0.1.0")
        .author("William Tarte <William.tarte6@gmail.com>")
        .subcommand(SubCommand::with_name("add")
            .help("Adds a local git repo to be tracked.")
            .arg(Arg::with_name("path").short("-p").long("path").allow_hyphen_values(true).takes_value(true).required(true))
            .arg(Arg::with_name("alias").short("-a").long("alias").allow_hyphen_values(true).takes_value(true).default_value("")))
        .arg(Arg::with_name("-d").allow_hyphen_values(true).takes_value(false))
        .subcommand(SubCommand::with_name("remove")
            .help("Removes a local git repo from being tracked.")
            .arg(Arg::with_name("name").short("-n").long("name").allow_hyphen_values(true).takes_value(true).required(true)))
        .subcommand(SubCommand::with_name("list")
            .help("Displays a list of tracked repositories.")
            .arg(Arg::with_name("verbose").short("-v").allow_hyphen_values(true).takes_value(false).required(false)))
        .subcommand(SubCommand::with_name("show")
            .help("Shows the status for the given repository.")
            .arg(Arg::with_name("name").short("-n").allow_hyphen_values(true).takes_value(true).required(true)))
        .arg(Arg::with_name("verbose").short("-v").allow_hyphen_values(true).takes_value(false).required(false))
        .get_matches();

    match matches.subcommand() {
        ("add", Some(args)) => { unimplemented!() },
        ("remove", Some(args)) => { unimplemented!() },
        ("list", Some(args)) => { unimplemented!() },
        ("show", Some(args)) => { unimplemented!() },
        _ => { unimplemented!() }
    }
}
