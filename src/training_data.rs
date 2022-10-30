use std::fs::File;
use std::io::{self, BufReader, BufRead, Read};

pub struct TrainingData {
    // training_data_file: File,
    pub reader: BufReader<File>,
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

    pub fn is_eof(&mut self, line: &mut String) -> bool {
        let mut buffer = String::new();

        let bytes = self.reader.by_ref().read_line(&mut buffer).unwrap();
        let mut result = false;
        if bytes == 0 {
            result = true;
        }
        *line = buffer;
        result
    }

    pub fn get_next_inputs(&self, input_vals: &mut Vec<f64>, line: &mut String) -> usize {
        input_vals.clear();
        let parsed_string: Vec<&str> = line.split(' ').collect();
        input_vals.push(parsed_string[1].parse::<f64>().unwrap());
        input_vals.push(parsed_string[2].parse::<f64>().unwrap());
        
        input_vals.len()
    }

    pub fn get_target_outputs(&self, target_vals: &mut Vec<f64>, line: &mut String) -> usize{
        target_vals.clear();
        let parsed_string: Vec<&str> = line.split(' ').collect();
        // println!("Parsed target: {}", parsed_string[1]);
        target_vals.push(parsed_string[1].parse::<f64>().unwrap());

        target_vals.len()
    }

    pub fn getline(&mut self, line: &mut String){
        let mut buffer = String::new();
        self.reader.by_ref().read_line(&mut buffer).unwrap();
        *line = buffer;
    }
}
