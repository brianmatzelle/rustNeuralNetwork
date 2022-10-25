mod trainingData;
use trainingData::TrainingData;

fn main() {
    let trainingData = TrainingData {
        file_path: String::from("./data/trainingData.txt")
    };

    let mut topology: Vec<u8> = Vec::new();
}
