mod training_data;
mod net;
mod neuron;
mod layer;
mod connection;
use std::io::BufRead;

use training_data::TrainingData;
use net::Net;

fn main() {
    // let training_data = TrainingData {
    //     file_path: String::from("./data/trainingData.txt")
    // };
    let mut training_data = TrainingData::new("./data/training_data.txt");

    let mut topology: Vec<u8> = Vec::new();
    training_data.get_topology(&mut topology);
    println!("Topology: {:?}", topology);
    let mut network = Net::new(&topology);
    let mut input_vals: Vec<f64> = Vec::new();
    let mut target_vals: Vec<f64> = Vec::new();
    let mut result_vals: Vec<f64> = Vec::new();
    let mut training_pass: i32 = 0;
    let mut line = String::new();
    while !training_data.is_eof(&mut line) {
        training_pass += 1;
        training_data.get_next_inputs(&mut input_vals, &mut line);
    }
}
