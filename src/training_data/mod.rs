pub mod training_couple;

use std::fs;
use training_couple::TrainingCouple;

pub fn training_set_from_file(path: String) -> Vec<TrainingCouple> {
    let contents = fs::read_to_string(path).expect("Error while reading the file");

    let mut data: Vec<TrainingCouple> = Vec::with_capacity(contents.lines().count());

    for line in contents.lines() {
        data.push(TrainingCouple::new(line.to_string()))
    }

    data
}
