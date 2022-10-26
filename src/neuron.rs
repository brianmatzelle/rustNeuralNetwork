use fastapprox::fast::tanh;
use crate::layer::Layer;
use crate::connection::Connection;

#[derive(Clone)]
pub struct Neuron {
    output_weights: Vec<Connection>,
    eta: f64,
    alpha: f64,
    output_val: f64,
    my_index: usize,
    gradient: f64,
}
pub fn random_weight() -> f64 { // done ? could bug
    let rng = rand::random::<f64>();
    let rand_weight = rng / 2147483647.0;
    rand_weight
}

impl Neuron {
    pub fn new(num_outputs: u8, neuron_num: u8) -> Neuron { // done
        let mut neuron = Neuron {
            output_weights: Vec::new(),
            eta: 0.15,
            alpha: 0.5,
            output_val: 0.0,
            my_index: 0,
            gradient: 0.0,
        };

        for _c in 0..num_outputs {
            neuron.output_weights.push(Connection::new());
            neuron.output_weights.last_mut().unwrap().weight = random_weight();
        }
        neuron
    }

    fn transfer_function(&mut self, x: f64) -> f64 { // done
        let y: f64 = tanh(x as f32) as f64;
        y
    }

    fn transfer_function_derivative(&mut self, x: f64) -> f64 { // done
        let y = 1.0 - x * x;
        y
    }


    fn sum_DOW(&self, next_layer: &Layer) -> f64 { // done
        let mut sum: f64 = 0.0;
        
        for n in 0..next_layer.len() - 1 {
            sum += self.output_weights[n].weight * next_layer.0[n].gradient;
        }
        sum
    }

    pub fn set_output_val(&mut self, val: f64) {

    }

    pub fn get_output_val(&mut self) -> f64 {

        let x = 0.0;    // NOT IMPLEMENTED
        x
    }

    pub fn calc_output_gradients(&mut self, target_vals: f64) {

    }

    pub fn calc_hidden_gradients(&mut self, next_layer: &Layer) {

    }
    
    pub fn update_input_weights(&self, prev_layer: &mut Layer) { // done
        for n in 0..prev_layer.len() {
            let neuron = &mut prev_layer.0[n];
            let old_delta_weight = neuron.output_weights[self.my_index].delta_weight;

            let new_delta_weight = 
                self.eta
                * neuron.get_output_val()
                * self.gradient
                * self.alpha
                * old_delta_weight;

            neuron.output_weights[self.my_index].delta_weight = new_delta_weight;
            neuron.output_weights[self.my_index].weight += new_delta_weight;
        }
    }
    
    pub fn feed_forward(&mut self, prev_layer: &Layer) {

    }
}