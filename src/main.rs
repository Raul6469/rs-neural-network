#![allow(dead_code)]
#![allow(unused_variables)]
mod neural_network;
mod training_data;

use neural_network::network::Network;

fn main() {
    let training_set = training_data::training_set_from_file("data/double.txt".to_string());
    let mut network = Network::new(vec!(1, 2, 2, 1));
    let results = network.predict(vec!(1f32));
    println!("{:?}", results);
}
