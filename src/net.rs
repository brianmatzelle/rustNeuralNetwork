use crate::neuron::Neuron;
use crate::layer::{Layer, self};
pub struct Net {    // done
    layers: Vec<Layer>,
    error: f64,
    recent_average_error: f64,
    recent_average_smoothing_factor: f64,
}

impl Net {
    pub fn new(topology: &Vec<u8>) -> Net { // done
        let mut net = Net {
            layers: Vec::new(),
            error: 0.0,
            recent_average_error: 0.0,
            recent_average_smoothing_factor: 100.0,
        };
        let num_layers = topology.len();
        for layer_num in 0..num_layers {
            println!("Layer created!");
            net.layers.push(Layer(Vec::new()));
            let num_outputs = if (layer_num == (topology.len() - 1)) { 0 } else { topology[layer_num + 1]}; // ternary operator equivalent
            
            let mut neuron_num = 0;
            while neuron_num <= topology[layer_num] {                                   // make neurons for each layer, <= to include the bias neuron
                net.layers[layer_num].push(Neuron::new(num_outputs, neuron_num.into()));
                println!("Made a Neuron!");
                neuron_num += 1;
            }
            println!();
            net.layers.last_mut().unwrap().last_mut().unwrap().set_output_val(1.0);                           // set the value of the bias neuron to 1.0 (last neuron in layer)
        }

        net                                    // return net
    }

    pub fn get_results(&mut self, result_vals: &mut Vec<f64>) { // done
        result_vals.clear();
        let mut n = 0;
        while n < (self.layers.last().unwrap().0.len() - 1) {
            result_vals.push(self.layers.last_mut().unwrap().0[n].get_output_val());
            n += 1;
        }
    }

    pub fn back_prop(&mut self, target_vals: &Vec<f64>) {   // done
        let mut output_layer = self.layers.last_mut().unwrap();
        self.error = 0.0;

        for n in 0..output_layer.len() - 1 {
            let delta = target_vals[n] - output_layer.0[n].get_output_val();    // output_layer.0 is the vector of neurons, we are taking n, a neuron
            self.error += delta * delta;
        }
        self.error /= (output_layer.len() - 1) as f64;
        self.error = self.error.sqrt();

        self.recent_average_error = 
            (self.recent_average_error * self.recent_average_smoothing_factor + self.error)
            / (self.recent_average_smoothing_factor + 1.0);
        
        for n in 0..output_layer.len() - 1 {
            output_layer.0[n].calc_output_gradients(target_vals[n]);
        }

        let mut layer_num = self.layers.len() - 2;
        for i in (0..layer_num).rev() {
            let next_layer = self.layers[i + 1].clone();
            let hidden_layer = &mut self.layers[i];

            for n in 0..hidden_layer.len() {
                hidden_layer.0[n].calc_hidden_gradients(&next_layer);
            }
        }

        layer_num = self.layers.len() - 1;
        for i in (0..layer_num).rev() {
            let layer = self.layers[i].clone();             // encountered ownership problems here, could bug
            let prev_layer = &mut self.layers[i - 1];

            for n in 0..layer.len() - 1 {
                layer.0[n].update_input_weights(prev_layer);
            }
        }
    }

    pub fn feed_forward(&mut self, input_vals: &Vec<f64>) { // done
        for i in 0..input_vals.len() - 1 {
            self.layers[0].0[i].set_output_val(input_vals[i]);
        }
        for layer_num in 1..self.layers.len() {
            let prev_layer = self.layers[layer_num - 1].clone();

            for n in 0..self.layers[layer_num].len() - 1 {
                self.layers[layer_num].0[n].feed_forward(&prev_layer);
            }
        }
    }

}