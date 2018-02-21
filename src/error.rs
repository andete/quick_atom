// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std;
use atom_syndication;
use failure::SyncFailure;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Herenbossen Error: {}", txt)]
    Error { txt: String, },
    #[fail(display = "IO Error: {:?}", error)]
    IO { error: std::io::Error, },
    #[fail(display = "Atom Error: {:?}", error)]
    Atom { error: SyncFailure<atom_syndication::Error>, },
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
