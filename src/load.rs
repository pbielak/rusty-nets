/// Networks loading API
use std::io::*;


fn load_network<'a>(filepath: String) -> Result<&'a str> {
    Err(Error::from(ErrorKind::NotFound))
}


#[cfg(test)]
#[path = "../tests/unit/load_tests.rs"]
mod load_tests;
