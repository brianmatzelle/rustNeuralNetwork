mod training_data;
mod net;
mod neuron;
mod layer;
mod connection;
use training_data::TrainingData;
use net::Net;

fn main() {
    let training_data = TrainingData {
        file_path: String::from("./data/trainingData.txt")
    };

    let mut topology: Vec<u8> = Vec::new();
    training_data.get_topology(&mut topology);
    println!("Topology: {:?}", topology);
    let mut network = Net::new(&topology);
}
