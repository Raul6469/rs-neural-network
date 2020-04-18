#![allow(dead_code)]
#![allow(unused_variables)]
mod neural_network;

use neural_network::network::Network;

fn main() {
    let mut network = Network::new(vec!(1, 2, 2, 1));
    let results = network.predict(vec!(1f32));
    println!("{:?}", results);
}
