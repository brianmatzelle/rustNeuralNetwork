use crate::neuron::Neuron;
use crate::layer::{Layer, self};

pub fn show_vector_vals(label: &str, v: &Vec<f64>) {
    println!("{} {:?} ", label, v);
}
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
                if neuron_num == topology[layer_num] { println!("Made a Neuron! -bias"); }
                else { println!("Made a Neuron!"); }
                neuron_num += 1;
            }
            println!();
            net.layers.last_mut().unwrap().last_mut().unwrap().set_output_val(1.0);                           // set the value of the bias neuron to 1.0 (last neuron in layer)
        }

        net                                    // return net
    }

    pub fn get_results(&mut self, result_vals: &mut Vec<f64>) { // done
        result_vals.clear();
        for n in 0..self.layers.last().unwrap().0.len() {
            result_vals.push(self.layers.last_mut().unwrap().0[n].get_output_val());
        }
    }

    pub fn back_prop(&mut self, target_vals: &Vec<f64>) {   // done
        let output_layer = self.layers.last_mut().unwrap();
        self.error = 0.0;

        for n in 0..output_layer.len() - 1 {
            let delta = target_vals[n] - output_layer.0[n].get_output_val();    // output_layer.0 is the vector of neurons, we are taking n, a neuron
            println!("delta[{}]: {}", n, delta);
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
        while layer_num > 0 {
            let (hidden_layer_vec, next_layer_vec) = self.layers.split_at_mut(layer_num+1);
            let hidden_layer = hidden_layer_vec.last_mut().unwrap();
            let next_layer = &mut next_layer_vec[0];

            for n in 0..hidden_layer.len() {
                hidden_layer.0[n].calc_hidden_gradients(next_layer);
            }
            layer_num -= 1;
        }

        layer_num = self.layers.len() - 1;
        while layer_num > 0 {
                let size = self.layers[layer_num].len();
                let (prev_layer_vec, layer_vec) = self.layers.split_at_mut(layer_num);
                let layer = &mut layer_vec[0];
                let prev_layer =  prev_layer_vec.last_mut().unwrap();   // should be correct

                for n in 0..size - 1 {
                    layer.0[n].update_input_weights(prev_layer);
                }
            layer_num -= 1;
        }
    }

    pub fn feed_forward(&mut self, input_vals: &Vec<f64>) { // done
        for i in 0..input_vals.len() {
            self.layers[0].0[i].set_output_val(input_vals[i]);
        }
        for layer_num in 1..self.layers.len() {
            // let prev_layer = self.layers[layer_num - 1].clone();
            let (prev_layer_vec, layer_vec) = self.layers.split_at_mut(layer_num);
            let prev_layer = prev_layer_vec.last_mut().unwrap();
            let layer = &mut layer_vec[0];

            // for n in 0..self.layers[layer_num].len() - 1 {
            //     self.layers[layer_num].0[n].feed_forward(&prev_layer);
            // }
            for n in 0..layer.len() - 1 {
                layer.0[n].feed_forward(prev_layer);
            }
        }
    }

    pub fn get_recent_average_error(&self) -> f64 {
        self.recent_average_error
    }

}