/// Network file reader
use std::fs;
use std::io;
use std::io::BufRead;
use std::num;
use std::path::PathBuf;

use crate::network::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    FileNotFound,
    MalformedFileFormat,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            io::ErrorKind::NotFound => Error::FileNotFound,
            _ => panic!(format!("Unknown error occurred: {:?}", e.kind())),
        }
    }
}

impl From<num::ParseIntError> for Error {
    fn from(_: num::ParseIntError) -> Self {
        Error::MalformedFileFormat
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(_: num::ParseFloatError) -> Self {
        Error::MalformedFileFormat
    }
}

pub trait NetworkReader {
    type N: NetworkNode;
    type E;

    fn read(&self, file: PathBuf) -> Result<Network<Self::N, Self::E>, Error>;
}

pub struct EdgeListReader {
    separator: char,
}

impl EdgeListReader {
    pub fn new(separator: char) -> EdgeListReader {
        EdgeListReader { separator }
    }
}

impl NetworkReader for EdgeListReader {
    type N = usize;
    type E = f64;

    fn read(&self, file: PathBuf) -> Result<Network<Self::N, Self::E>, Error> {
        let f = fs::File::open(file)?;
        let reader = io::BufReader::new(&f);

        let mut net = Network::new();

        for line in reader.lines() {
            let edge_raw: String = line?;
            let edge: Vec<&str> = edge_raw.split(self.separator).collect();

            let from_node: usize = edge[0].parse()?;
            let to_node: usize = edge[1].parse()?;
            let edge_data: f64 = edge[2].parse()?;

            net.add_edge(from_node, to_node, edge_data);
        }

        Ok(net)
    }
}

#[cfg(test)]
#[path = "../tests/unit/reader_tests.rs"]
mod reader_tests;
