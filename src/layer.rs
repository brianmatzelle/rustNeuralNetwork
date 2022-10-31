use crate::neuron::Neuron;
#[derive(Debug)]
pub struct Layer(pub Vec<Neuron>);              // vector with neurons
impl Layer {                            // same as typedef in c++
    pub fn push(&mut self, n: Neuron) {     // so we can use .push() from the vector library
        self.0.push(n);                 // self.0 is self's vector
    }                                   // realized after that this is redundant
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn last_mut(&mut self) -> Option<&mut Neuron> {
        self.0.last_mut()
    }
    pub fn clone(&mut self) -> Layer {
        let layer = Layer(self.0.clone());
        layer
    }
}