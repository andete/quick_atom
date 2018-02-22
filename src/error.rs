// (c) 2018 Joost Yervante Damad <joost@damad.be>

use std;
use atom_syndication;
use failure::SyncFailure;

/// Error
#[derive(Fail, Debug)]
pub enum Error {
    /// Quick Atom Error
    #[fail(display = "Quick Atom Error: {}", txt)]
    Error {
        /// `String` describing the `Error::Error`
        txt: String,
    },
    /// IO Error
    #[fail(display = "IO Error: {:?}", error)]
    IO {
        /// the containing `std::io::Error`
        error: std::io::Error,
    },
    /// Atom Error
    #[fail(display = "Atom Error: {:?}", error)]
    Atom {
        /// the containing `atom_syndication::Error
        error: SyncFailure<atom_syndication::Error>,
    },
}

impl From<std::io::Error> for Error {
    fn from(e:std::io::Error) -> Error {
        Error::IO {
            error: e
        }
    }
}

impl From<atom_syndication::Error> for Error {
    fn from(e:atom_syndication::Error) -> Error {
        Error::Atom {
            error: SyncFailure::new(e)
        }
    }
}

impl From<String> for Error {
    fn from(e:String) -> Error {
        Error::Error {
            txt: e
        }
    }
}


impl<'a> From<&'a str> for Error {
    fn from(e:&'a str) -> Error {
        Error::Error {
            txt: e.into()
        }
    }
}
