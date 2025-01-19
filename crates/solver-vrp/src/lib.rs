// This crate is WIP.
//
// Notes:
//   - Manage stop assignments in `Model`.
//   - Store references to `Stop`s in `Model`. `PlanUnit` and `Vehicle` shouldn't need copies of each `Stop` if they aren't modified.
//   - Implement traits for interfaces like `PlanUnit`s with different implementations, operators, expressions, etc.
//   - Reference parsed input data directly in `Input`.
//   - Only ["stop-id"] single-precedence inputs for stops.
//   - Only {"dimension": f64} quantity inputs for stops.
//   - Only f64 location coordinate precision inputs for stops.
//   - Only `u64` [epoch seconds, epoch seconds] as a fixed start and end time for start time window inputs for stops.
//   - Only f64 distance matrix inputs.
//   - Only inputs without options used.
//   - Include vehicle type data.

use model::Model;
use solution::Solution;

mod model;
mod schema;
mod solution;

type Id = String;
type Index = usize;

struct Solver {
    model: Model,
}

impl Solver {
    fn new(model: Model) -> Self {
        Self { model }
    }

    fn solve(&self, options: Options) -> Solution {
        let initial_solution = Solution::new(&self.model);

        for _ in 0..options.iterations {
            unimplemented!()
        }

        initial_solution
    }

    fn model(&self) -> &Model {
        &self.model
    }
}

struct Options {
    iterations: u64,
}

impl Default for Options {
    fn default() -> Self {
        Self { iterations: 10 }
    }
}

#[test]
fn test_solver() {
    use schema::{
        InitialStop, Input, Location as InputLocation, Stop as InputStop, Vehicle as InputVehicle,
    };
    use std::collections::HashMap;

    let model = Model::from(Input {
        stops: vec![
            InputStop {
                id: String::from("pickup"),
                precedes: Some(vec![String::from("delivery")]),
                quantity: HashMap::from([(String::from("count"), -1.)]),
                start_time_windows: [0; 2],
                location: InputLocation { lat: 0., lon: 0. },
            },
            InputStop {
                id: String::from("delivery"),
                precedes: None,
                quantity: HashMap::from([(String::from("count"), 1.)]),
                start_time_windows: [0; 2],
                location: InputLocation { lat: 0., lon: 0. },
            },
        ],
        vehicles: vec![InputVehicle {
            id: String::from("vehicle"),
            capacity: HashMap::from([(String::from("count"), 1.)]),
            speed: 0.,
            initial_stops: Some(vec![
                InitialStop {
                    id: String::from("pickup"),
                },
                InitialStop {
                    id: String::from("pickup"),
                },
            ]),
        }],
        distance_matrix: vec![vec![0., 1.], vec![1., 0.]],
        options: None,
    });

    let solution = Solver::new(model).solve(Options {
        iterations: 0,
        ..Default::default()
    });

    assert_eq!(solution.value(), 0.)
}
