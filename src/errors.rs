use std::fmt::{Display, Formatter};
use std::error::Error;

#[derive(Debug)]
pub enum Errors {
    FileIsDirError,
    FileNotExistsError,
    InvalidArgsException,
    GeneralError,
}
impl Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Errors::FileIsDirError => { write!(f, "`{:?}`: File is a directory.", self)}
            Errors::FileNotExistsError => { write!(f, "`{:?}`: File does not exist.", self)}
            Errors::InvalidArgsException => { write!(f, "`{:?}`: The first arg should be the path to the file, the second the pattern to search for, \
            and the third optionally the new string to replace with.", self)}
            Errors::GeneralError => { write!(f, "`{:?}`: Something went wrong!", self)}
        }
    }
}
impl Error for Errors {}