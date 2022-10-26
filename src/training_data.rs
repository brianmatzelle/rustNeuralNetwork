use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct TrainingData {
    training_data_file: File,
    reader: BufReader<File>,
}

impl TrainingData { 
    pub fn new(filename: &str) -> TrainingData {
        let t = TrainingData {
            training_data_file: File::open(filename).unwrap(),
            reader: BufReader::new(File::open(filename).unwrap()),
        };
        t
    }
    pub fn get_topology(&self, topology: &mut Vec<u8>)  {
        let line = self.reader.lines().nth(0).unwrap().unwrap();
        for c in line.chars() {
            if c.is_numeric() {
                topology.push(c as u8 - 0x30);
            }
        }
    }
}
