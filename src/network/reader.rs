/// Network file reader
use std::fs;
use std::io;
use std::io::BufRead;
use std::num;
use std::path::PathBuf;

use crate::network::model::*;

#[derive(Debug, PartialEq)]
pub enum ReaderError {
    FileNotFound,
    MalformedFileFormat(String),
}

impl From<io::Error> for ReaderError {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            io::ErrorKind::NotFound => ReaderError::FileNotFound,
            _ => panic!(format!("Unknown error occurred: {:?}", e.kind())),
        }
    }
}

impl From<num::ParseIntError> for ReaderError {
    fn from(_: num::ParseIntError) -> Self {
        ReaderError::MalformedFileFormat("Couldn't parse int".to_string())
    }
}

impl From<num::ParseFloatError> for ReaderError {
    fn from(_: num::ParseFloatError) -> Self {
        ReaderError::MalformedFileFormat("Couldn't parse float".to_string())
    }
}

pub trait NetworkReader {
    type N: NetworkNode;
    type E: Copy;

    fn read(&self, file: PathBuf) -> Result<Network<Self::N, Self::E>, ReaderError>;
}

pub struct EdgeListReader {
    separator: char,
    directed: bool,
}

impl EdgeListReader {
    pub fn new(separator: char, directed: bool) -> EdgeListReader {
        EdgeListReader {
            separator,
            directed,
        }
    }
}

impl NetworkReader for EdgeListReader {
    type N = usize;
    type E = f64;

    fn read(&self, file: PathBuf) -> Result<Network<Self::N, Self::E>, ReaderError> {
        let f = fs::File::open(file)?;
        let reader = io::BufReader::new(&f);

        let mut net = Network::new(self.directed);

        for line in reader.lines() {
            let edge_raw: String = line?;
            let edge: Vec<&str> = edge_raw.split(self.separator).collect();

            if edge.len() < 3 {
                return Err(ReaderError::MalformedFileFormat(
                    "Not enough elements".to_string(),
                ));
            }

            let from_node: usize = edge[0].parse()?;
            let to_node: usize = edge[1].parse()?;
            let edge_data: f64 = edge[2].parse()?;

            net.add_edge(from_node, to_node, edge_data);
        }

        Ok(net)
    }
}

#[cfg(test)]
#[path = "../../tests/unit/network/reader_tests.rs"]
mod reader_tests;
