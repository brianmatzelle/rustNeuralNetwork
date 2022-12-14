// use crate::neuron::Neuron::random_weight;   

#[derive(Debug, Clone)]
pub struct Connection {
    pub weight: f64,
    pub delta_weight: f64,
}

// impl Connection {
//     pub fn new() -> Connection {
//         Connection {
//             weight: random_weight(),
//             delta_weight: 1.0,
//         }
//     }
// }