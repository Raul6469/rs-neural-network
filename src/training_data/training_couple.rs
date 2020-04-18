#[derive(Debug)]
pub struct TrainingCouple {
    features: Vec<f32>,
    labels: Vec<f32>,
}

impl TrainingCouple {
    pub fn new(line: String) -> TrainingCouple {
        let mut res: Vec<Vec<f32>> = line
            .split("|")
            .map(move |group| {
                group
                    .split(",")
                    .map(move |val| {
                        val.parse::<f32>().expect("Unable to parse to float")
                    })
                    .collect()
            })
            .collect();

        TrainingCouple {
            features: res.swap_remove(0),
            labels: res.swap_remove(0),
        }
    }
}
