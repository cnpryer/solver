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
    fn new() -> Self {
        Self {
            model: Model::new(),
        }
    }

    fn solve(&self) -> Solution {
        todo!()
    }

    fn model(&self) -> &Model {
        &self.model
    }
}
