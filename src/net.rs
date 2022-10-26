use crate::neuron::Neuron;

struct Layer(Vec<Neuron>);              // vector with neurons
impl Layer {                            // same as typedef in c++
    fn push(&mut self, n: Neuron) {     // so we can use .push() from the vector library
        self.0.push(n);                 // self.0 is self's vector
    }                                   // realized after that this is redundant
}
pub struct Net {
    layers: Vec<Layer>,
    error: f64,
    recent_average_error: f64,
    recent_average_smoothing_factor: f64,
}

impl Net {
    pub fn new(topology: &Vec<u8>) -> Net {
        let mut net = Net {
            layers: Vec::new(),
            error: 0.0,
            recent_average_error: 0.0,
            recent_average_smoothing_factor: 0.0,
        };
        let num_layers = topology.len();
        for layer_num in 0..num_layers {
            println!("Layer created!");
            net.layers.push(Layer(Vec::new()));
            let num_outputs = if (layer_num == (topology.len() - 1)) { 0 } else { topology[layer_num + 1]}; // ternary operator equivalent
            
            let mut neuron_num = 0;
            while neuron_num <= topology[layer_num] {                                   // make neurons for each layer, <= to include the bias neuron
                net.layers[layer_num].push(Neuron::new(num_outputs, neuron_num));
                println!("Made a Neuron!");
                neuron_num += 1;
            }
            println!();
            net.layers.last_mut().unwrap().0.last_mut().unwrap().set_output_val(1.0);                           // set the value of the bias neuron to 1.0 (last neuron in layer)
        }

        net                                    // return net
    }
}