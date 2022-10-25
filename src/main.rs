mod training_data;
use training_data::TrainingData;

fn main() {
    let training_data = TrainingData {
        file_path: String::from("./data/trainingData.txt")
    };

    let topology: Vec<u8> = Vec::new();
    training_data.get_topology(&topology);
}
