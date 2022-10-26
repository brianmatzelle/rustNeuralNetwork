use crate::layer::Layer;
use crate::connection::Connection;

#[derive(Clone)]
pub struct Neuron {
    output_weights: Connection,
    eta: f64,
    alpha: f64,
    output_val: f64,
    my_index: u16,
    gradient: f64,
}

impl Neuron {
    pub fn new(num_outputs: u8, neuron_num: u8) -> Neuron {
        Neuron {
            output_weights: Connection::new(),
            eta: 0.0,
            alpha: 0.0,
            output_val: 0.0,
            my_index: 0,
            gradient: 0.0,
        }
    }
    
    fn transfer_function(&mut self, x: f64) -> f64 {

    }

    fn transfer_function_derivative(&mut self, x: f64) -> f64 {

    }

    fn random_weight(&mut self) -> f64 {

    }

    fn sum_DOW(&mut self, next_layer: &Layer) -> f64 {

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
    
    pub fn update_input_weights(&mut self, prev_layer: &Layer) {

    }
    
    pub fn feed_forward(&mut self, prev_layer: &Layer) {

    }
}