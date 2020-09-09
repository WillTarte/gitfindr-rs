use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use clap::{App, Arg, SubCommand};
use serde::{Deserialize, Serialize};

use err::{NotARepositoryError, RepoAlreadyExistsError};

use crate::err::RepoDoesNotExistError;

mod err;

/// Name of the config file
const CONFIG_NAME: &str = "gitfnder";
/// File extension for git repo
const GIT_FILE: &str = ".git";

type GFResult<T> = Result<T, Box<dyn Error>>;

/// Holds data about a repository
#[derive(Debug, Serialize, Deserialize)]
struct RepoData {
    name: String,
    path: PathBuf,
}

impl RepoData {
    /// Creates new data object for a repo
    ///
    /// # Arguments
    /// * `name` - A string slice representing the repositories name.
    /// TODO: default repo name fallback if not provided
    /// * `path` - A string slice representing the absolute path to the repository's directory.
    ///
    ///  # Returns
    /// A result object with the RepoData on success, or an Err variant with a Box\<dyn Error\> on failure.
    fn new(name: &str, path: &str) -> GFResult<Self> {
        let mut repo_data = RepoData::default();
        repo_data.name = name.to_string();
        repo_data.path = match validate_repo(path) {
            Ok(_) => match PathBuf::from_str(path) {
                Ok(buf) => buf,
                Err(err) => return Result::Err(Box::new(err)),
            },
            Err(err) => return Result::Err(err),
        };
        Ok(repo_data)
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

/// Config file struct
#[derive(Debug, Serialize, Deserialize)]
struct GitFindrConfig {
    repos: HashMap<String, RepoData>,
}

impl GitFindrConfig {
    /// Adds a repository to the config
    ///
    /// # Arguments
    /// * `repo` - A RepoData object to add to the config
    ///
    /// # Returns
    /// Result Ok variant if successfully added
    fn add_repo(&mut self, repo: RepoData) -> GFResult<()> {
        return if self.repos.contains_key(&repo.name) {
            Err(Box::new(RepoAlreadyExistsError))
        } else {
            self.repos.insert(repo.name.clone(), repo);
            Ok(())
        };
    }

    fn remove_repo(&mut self, name: &str) -> GFResult<()> {
        return if !self.repos.contains_key(name) {
            Err(Box::new(RepoDoesNotExistError))
        } else {
            self.repos.remove(name);
            Ok(())
        };
    }

    fn get_repo(&self, name: &str) -> Option<&RepoData> {
        self.repos.get(name)
    }
}

impl Default for GitFindrConfig {
    fn default() -> Self {
        GitFindrConfig {
            repos: HashMap::new(),
        }
    }
}

/// Validates that the given string slice is a valid path that points to a repository.
///
/// # Arguments
/// * `path_str` - A string slice representing the absolute path to a repository
///
/// # Returns
/// A result object with an empty Ok variant on success, or an Err variant with a Box\<dyn Error\> on failure.
fn validate_repo(path_str: &str) -> self::GFResult<()> {
    let path_buf: PathBuf = match PathBuf::from_str(path_str) {
        Ok(buf) => buf,
        Err(err) => return Result::Err(Box::new(err)),
    };

    match fs::read_dir(path_buf) {
        Ok(dir_it) => {
            for entry_res in dir_it {
                match entry_res {
                    Ok(entry) => {
                        if entry.file_name().eq(GIT_FILE) {
                            return Result::Ok(());
                        }
                    }
                    Err(err) => return Result::Err(Box::new(err)),
                }
            }
        }
        Err(err) => return Result::Err(Box::new(err)),
    }
    Result::Err(Box::new(NotARepositoryError))
}

//TODO does rayon have parallel iterators for directories?
fn parse_directory(dir_path: &str) -> self::GFResult<Vec<RepoData>> {
    match fs::read_dir(dir_path) {
        Ok(read_dir) => {
            let res: Result<Vec<RepoData>, Box<dyn Error>> = read_dir
                .into_iter()
                .map(|dir_entry_res| {
                    let res: Result<RepoData, Box<dyn Error>> = return match dir_entry_res {
                        Ok(dir_entry) => {
                            if dir_entry.file_name().eq(GIT_FILE) {
                                RepoData::new("name", dir_entry.path().to_str().unwrap())
                            } else {
                                match dir_entry.metadata() {
                                    Ok(meta) => {
                                        if meta.is_dir() {
                                            todo!();
                                            fs::read_dir(dir_entry.path())
                                        }

                                        RepoData::new("", "")
                                    }
                                    Err(err) => Err(Box::from(err)),
                                }
                            }
                        }
                        Err(err) => Err(Box::from(err)),
                    };
                })
                .collect();
        }
        Err(err) => eprintln!("{:?}", err),
    }
    Ok(vec![])
}

fn main() {
    let mut config: GitFindrConfig = confy::load(CONFIG_NAME).unwrap();

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
                        .required(true),
                )
                .arg(
                    Arg::with_name("alias")
                        .short("-a")
                        .long("alias")
                        .allow_hyphen_values(true)
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("-d")
                        .allow_hyphen_values(true)
                        .takes_value(false),
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
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .help("Displays a list of tracked repositories.")
                .arg(
                    Arg::with_name("verbose")
                        .short("-v")
                        .allow_hyphen_values(true)
                        .takes_value(false)
                        .required(false),
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
                        .required(true),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("-v")
                        .allow_hyphen_values(true)
                        .takes_value(false)
                        .required(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("add", Some(args)) => {
            if args.is_present("-d") {}

            match (args.value_of("alias"), args.value_of("path")) {
                (Some(alias), Some(path)) => match RepoData::new(alias, path) {
                    Ok(repo) => match config.add_repo(repo) {
                        Ok(_) => {}
                        Err(err) => eprintln!("{}", err),
                    },
                    Err(err) => eprintln!("{}", err),
                },
                (_, None) => eprintln!("User tried to add a repo with no path"),
                (None, _) => eprintln!("User tried to add a repo with no name"),
            }
        }

        ("remove", Some(args)) => match args.value_of("name") {
            Some(name) => {
                config.remove_repo(name);
            }
            None => eprintln!("User did give a repo to remove."),
        },

        ("list", Some(args)) => {
            if args.is_present("-v") || args.is_present("verbose") {
                todo!("handle verbose output")
            } else {
                match config.repos.is_empty() {
                    true => println!("No repos to show!"),
                    false => {
                        for (key, val) in config.repos.iter() {
                            println!("{} : {:?}", key, val);
                        }
                    }
                }
            }
        }

        ("show", Some(args)) => {
            if args.is_present("-v") || args.is_present("verbose") {
                todo!("handle verbose output")
            } else {
                match args.value_of("name") {
                    Some(name) => match config.get_repo(name) {
                        Some(repo) => println!("{:?}", repo),
                        None => eprintln!("No repo to show for name {}", name),
                    },
                    None => eprintln!("No repo name was passed!"),
                }
            }
        }

        _ => eprintln!("No valid command was used"),
    }

    match confy::store(CONFIG_NAME, config) {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    }
}
