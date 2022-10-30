mod training_data;
mod net;
mod neuron;
mod layer;
mod connection;
use std::io::BufRead;

use training_data::TrainingData;
use net::Net;

use crate::net::show_vector_vals;

fn main() {
    // let training_data = TrainingData {
    //     file_path: String::from("./data/trainingData.txt")
    // };
    let mut training_data = TrainingData::new("./data/sample_data.txt");

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
        println!();
        print!("Pass {}", training_pass);
        if training_data.get_next_inputs(&mut input_vals, &mut line) != topology[0].into() {
            break;
        }
        show_vector_vals(": Inputs:", &input_vals);
        network.feed_forward(&input_vals);

        network.get_results(&mut result_vals);
        show_vector_vals("Outputs", &result_vals);
        
        training_data.getline(&mut line);
        training_data.get_target_outputs(&mut target_vals, &mut line);
        show_vector_vals("Targets:", &target_vals);

        network.back_prop(&target_vals);

        println!("Net recent average error: {}", network.get_recent_average_error());
        // println!("input: {:?}, target: {:?}", input_vals, target_vals);
    }
    println!("Done");
}
