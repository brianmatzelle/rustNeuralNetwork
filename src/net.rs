use crate::neuron::Neuron;

struct Layer(Vec<Neuron>);
impl Layer {
    fn new() -> Self {
        
    }
}
type Layer = Vec<Neuron>;
pub struct Net {
    layers: Vec<Layer>,
    error: f64,
    recent_average_error: f64,
    recent_average_smoothing_factor: f64,
}

impl Net {
    pub fn new(topology: &Vec<u8>) -> Net {
        let net = Net {
            layers: Vec::new(),
            error: 0.0,
            recent_average_error: 0.0,
            recent_average_smoothing_factor: 0.0,
        };
        let num_layers = topology.len();
        for layer_num in 0..num_layers {
            let layer = Layer;
            net.layers.push(layer);
        }

        net
    }
}