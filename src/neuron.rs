pub struct Neuron {
    eta: f64,
    alpha: f64,
    output_val: f64,
    my_index: u16,
    gradient: f64,
}

impl Neuron {
    pub fn new(num_outputs: u8, neuron_num: u8) -> Neuron {
        Neuron {
            eta: 0.0,
            alpha: 0.0,
            output_val: 0.0,
            my_index: 0,
            gradient: 0.0,
        }
    }
    
    pub fn set_output_val(&mut self, val: f32) {

    }
}