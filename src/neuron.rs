pub struct Neuron {
    eta: f64,
    alpha: f64,
    output_val: f64,
    myIndex: u16,
    gradient: f64,
}

impl Neuron {
    pub fn new() -> Neuron {
        Neuron {
            eta: 0.0,
            alpha: 0.0,
            output_val: 0.0,
            myIndex: 0,
            gradient: 0.0,
        }
    }
}