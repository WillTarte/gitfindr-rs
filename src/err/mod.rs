use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// Error message for non-repo directories
const NOT_A_REPOSITORY: &str = "The given directory is not a valid repository.";
/// Error message for repo that already exists
const REPO_ALREADY_EXISTS: &str = "The given repository already exists.";
/// Error message for repo that does not exist
const REPO_DOES_NOT_EXIST: &str = "The given repository does not exist";

#[derive(Debug)]
pub struct NotARepositoryError;

impl Display for NotARepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", NOT_A_REPOSITORY)
    }
}

impl Error for NotARepositoryError {}


#[derive(Debug)]
pub struct RepoAlreadyExistsError;

impl Display for RepoAlreadyExistsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", REPO_ALREADY_EXISTS)
    }
}

impl Error for RepoAlreadyExistsError {}


#[derive(Debug)]
pub struct RepoDoesNotExistError;

impl Display for RepoDoesNotExistError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", REPO_DOES_NOT_EXIST)
    }
}

impl Error for RepoDoesNotExistError {}