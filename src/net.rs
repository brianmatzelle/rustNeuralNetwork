use crate::neuron::Neuron;

type Layer = Vec<Neuron>;
pub struct Net {
    error: f64,
    recent_average_error: f64,
    recent_average_smoothing_factor: f64,
}

impl Net {
    pub fn new() -> Net {
        Net {
            error: 0.0,
            recent_average_error: 0.0,
            recent_average_smoothing_factor: 0.0,
        }
    }
}