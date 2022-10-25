use std::fs;
use std::env;

pub struct TrainingData {
    pub file_path: String,
}

impl TrainingData { 
    pub fn get_topology(&self, topology: &Vec<u8>)  {
        let path: &str = &self.file_path[..];
        let contents = fs::read_to_string(path)
            .expect("Should have been able to read the file");
        
        let mut lines = contents.lines();
        // println!("line[0]: {}", lines.nth(0).unwrap());
        topology.push(lines.nth(0).expect("Topology instruction should be given here"));
    }
}
