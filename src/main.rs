use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use clap::{App, Arg, SubCommand};
use serde::{Deserialize, Serialize};

static CONFIG_NAME: &str = "gitfnder";

#[derive(Debug, Serialize, Deserialize)]
struct RepoData {
    name: String,
    path: PathBuf,
}

impl RepoData {
    fn new(name: &str, path: &str) -> Self {
        RepoData {
            name: name.to_string(),
            path: match PathBuf::from_str(path) {
                Ok(pathbuf) => pathbuf,
                Err(_) => panic!("Invalid path"),
            },
        }
    }
}

impl Default for RepoData {
    fn default() -> Self {
        RepoData {
            name: String::new(),
            path: PathBuf::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GitFindrConfig {
    repos: HashMap<String, RepoData>,
}

impl Default for GitFindrConfig {
    fn default() -> Self {
        GitFindrConfig {
            repos: HashMap::new(),
        }
    }
}

fn main() {
    let mut config: GitFindrConfig = confy::load(CONFIG_NAME).unwrap();
    println!("{:?}", &config);

    let matches = App::new("Gitfindr-rs")
        .about("Helps you manage your local git repositories.")
        .version("0.1.0")
        .author("William Tarte <William.tarte6@gmail.com>")
        .subcommand(
            SubCommand::with_name("add")
                .help("Adds a local git repo to be tracked.")
                .arg(
                    Arg::with_name("path")
                        .short("-p")
                        .long("path")
                        .allow_hyphen_values(true)
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("alias")
                        .short("-a")
                        .long("alias")
                        .allow_hyphen_values(true)
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("-d")
                        .allow_hyphen_values(true)
                        .takes_value(false)
                )
                .help("When you add a directory possible containing multiple repositories."),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .help("Removes a local git repo from being tracked.")
                .arg(
                    Arg::with_name("name")
                        .short("-n")
                        .long("name")
                        .allow_hyphen_values(true)
                        .takes_value(true)
                        .required(true)
                )
        )
        .subcommand(
            SubCommand::with_name("list")
                .help("Displays a list of tracked repositories.")
                .arg(
                    Arg::with_name("verbose")
                        .short("-v")
                        .allow_hyphen_values(true)
                        .takes_value(false)
                        .required(false)
                ),
        )
        .subcommand(
            SubCommand::with_name("show")
                .help("Shows the status for the given repository.")
                .arg(
                    Arg::with_name("name")
                        .short("-n")
                        .allow_hyphen_values(true)
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("-v")
                        .allow_hyphen_values(true)
                        .takes_value(false)
                        .required(false)
                )
        )
        .get_matches();

    match matches.subcommand() {
        ("add", Some(args)) => {
            println!("{:?}", args);

            if args.is_present("-d") {
                todo!("check directory for repos");
            }

            match (args.value_of("alias"), args.value_of("path")) {
                (Some(alias), Some(path)) => {
                    if config.repos.contains_key(alias) {
                        println!("Alias already exists!");
                        return;
                    }
                    config.repos.insert(alias.to_string(), RepoData::new(alias, path));
                }
                (_, None) => panic!("User tried to add a repo with no path"),
                (None, _) => panic!("User tried to add a repo with no name"),
            }
        }

        ("remove", Some(args)) => {
            println!("{:?}", args);

            match args.value_of("name") {
                Some(name) => { config.repos.remove(name); },
                None => panic!("User did give a repo to remove.")
            }
            todo!("add some extra output?")
        }

        ("list", Some(args)) => {
            println!("{:?}", args);
            if args.is_present("-v") {
                todo!("handle verbose output")
            } else {
                for (key, val) in config.repos.iter() {
                    println!("{} : {:?}", key, val);
                }
            }
        }

        ("show", Some(args)) => {
            println!("{:?}", args);
            unimplemented!()
        },

        _ => println!("None"),
    }

    match confy::store(CONFIG_NAME, config) {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    }
}
