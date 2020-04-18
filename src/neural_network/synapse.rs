use std::rc::Rc;
use std::cell::RefCell;

use super::neuron::Neuron;

#[derive(Debug)]
pub struct Synapse {
    predecessor: Rc<RefCell<Neuron>>,
    weight: f32,
}

impl Synapse {
    pub fn new(neuron: &Rc<RefCell<Neuron>>) -> Synapse {
        Synapse {
            predecessor: Rc::clone(neuron),
            weight: 0.4f32,
        }
    }

    pub fn get_weighted_input(&self) -> f32 {
        self.weight * self.predecessor.borrow().get_inner_value()
    }
}
