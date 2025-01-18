// This crate is WIP.
//
// Notes:
//   - Manage stop assignments in `Model`.
//   - Store references to `Stop`s in `Model`. `PlanUnit` and `Vehicle` shouldn't need copies of each `Stop` if they aren't modified.
//   - Implement traits for interfaces like `PlanUnit`s with different implementations, operators, expressions, etc.
//   - Reference parsed input data directly in `Input`.
//   - Focusing on ["stop-id"] single-precedence inputs for stops.
//   - Focusing on {"count": i64} quantity inputs for stops.
//   - Focusing on ["date", "date"] start time window inputs for stops.
//   - Focusing on `u64` [epoch seconds, epoch seconds] as a fixed start and end time for start time window inputs for stops.
//   - Focusing on default `Number` and `Float` parsing.
//   - Focusing on inputs without options used.
//   - Include vehicle type data.

use model::Model;
use solution::Solution;

mod model;
mod schema;
mod solution;
mod types;

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
