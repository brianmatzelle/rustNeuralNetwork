use std::fs::File;
use std::io::{self, BufReader, BufRead, Read};

pub struct TrainingData {
    // training_data_file: File,
    reader: BufReader<File>,
}

impl TrainingData { 
    pub fn new(filename: &str) -> TrainingData {
        let t = TrainingData {
            // training_data_file: File::open(filename).unwrap(),
            reader: BufReader::new(File::open(filename).unwrap()),
        };
        t
    }

    pub fn get_topology(&mut self, topology: &mut Vec<u8>) {
        let line = self.reader.by_ref().lines().nth(0).unwrap().unwrap();
        for c in line.chars() {
            if c.is_numeric() {
                topology.push(c as u8 - 0x30);
            }
        }
    }

    pub fn is_eof(&mut self) -> bool {
        let mut empty: &[u8] = &[];
        let mut buffer = String::new();

        let bytes = self.reader.by_ref().read_line(&mut buffer).unwrap();
        let mut result = false;
        if bytes == 0 {
            result = true;
        }
        result
    }

    pub fn get_next_inputs(&mut self, input_vals: &mut Vec<f64>) -> usize {
        input_vals.clear();
        let mut buffer = String::new();
        let mut line = self.reader.by_ref().read_line(&mut buffer).unwrap();
        println!("buffer: {}, line: {}", buffer, line);
        println!();
        return input_vals.len();
    }
}
