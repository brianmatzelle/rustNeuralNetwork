use utils::math::sigmoid;
use crate::layer::Layer;
use crate::connection::Connection;

#[derive(Debug, Clone)]
pub struct Neuron {
    output_weights: Vec<Connection>,
    // eta: f64,
    // alpha: f64,
    output_val: f64,
    my_index: usize,
    gradient: f64,
}

static eta: f64 = 0.15;
static alpha: f64 = 0.5;

impl Neuron {
    pub fn new(num_outputs: u8, my_index: usize) -> Neuron { // done
        let mut neuron = Neuron {
            output_weights: Vec::new(),
            // eta: 0.15,
            // alpha: 0.5,
            output_val: 0.0,
            my_index,
            gradient: 0.0,
        };

        for _ in 0..num_outputs as usize {
            neuron.output_weights.push(Connection { weight: Self::random_weight(), delta_weight: 0.0 })
        }
        neuron
    }

    fn random_weight() -> f64 { // done ? could bug
        let rng = rand::random::<f64>();
        println!("rng:{}", rng);
        let rand_weight = rng / 2147483647.0;
        rand_weight
    }

    fn transfer_function(x: f64) -> f64 { // done
        sigmoid(x)
    }

    fn transfer_function_derivative(x: f64) -> f64 { // done
        (1.0 - x) * x
    }


    fn sum_dow(&self, next_layer: &Layer) -> f64 { // done
        let mut sum: f64 = 0.0;
        
        for n in 0..next_layer.len() - 1 {
            sum += self.output_weights[n].weight * next_layer[n].gradient;
        }
        sum
    }

    pub fn set_output_val(&mut self, val: f64) {    // done
        self.output_val = val;
    }

    pub fn get_output_val(&self) -> f64 {           // done
        self.output_val
    }

    pub fn calc_output_gradients(&mut self, target_val: f64) { // done
        let delta = target_val - self.output_val;
        self.gradient = delta * Self::transfer_function_derivative(self.output_val);
    }

    pub fn calc_hidden_gradients(&mut self, next_layer: &Layer) { // done
        let dow = self.sum_dow(next_layer);
        self.gradient = dow * Self::transfer_function_derivative(self.output_val);
    }
    
    pub fn update_input_weights(&self, prev_layer: &mut Layer) { // done
        for n in 0..prev_layer.len() {
            let neuron = &mut prev_layer[n];
            let old_delta_weight = neuron.output_weights[self.my_index].delta_weight;
            
            let new_delta_weight = 
                eta
                * neuron.get_output_val()
                * self.gradient
                * alpha
                * old_delta_weight;

            neuron.output_weights[self.my_index].delta_weight = new_delta_weight;
            neuron.output_weights[self.my_index].weight += new_delta_weight;
        }
    }
    
    pub fn feed_forward(&mut self, prev_layer: &Layer) {
        let mut sum: f64 = 0.0;

        for n in 0..prev_layer.len() {
            sum += prev_layer[n].get_output_val()
            * prev_layer[n].output_weights[self.my_index].weight;
        }
        // println!();
        // println!("index[{}] sum: {}", self.my_index, sum);

        self.output_val = Self::transfer_function(sum);
        // println!("output: {}", self.output_val);
        // println!();
    }
}