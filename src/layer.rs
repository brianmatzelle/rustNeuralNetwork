use crate::neuron::Neuron;
pub(crate) type Layer = Vec<Neuron>;
#[allow(non_snake_case)]
pub fn Layer() -> Layer {
    Vec::new()
}