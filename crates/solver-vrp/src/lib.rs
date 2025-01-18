// This crate is WIP.
//
// Notes:
//   - Manage stop assignments in `Model`.
//   - Store references to `Stop`s in `Model`. `PlanUnit` and `Vehicle` shouldn't need copies of each `Stop` if they aren't modified.
//   - Implement traits for interfaces like `PlanUnit`s with different implementations, operators, expressions, etc.

use model::Model;
use solution::Solution;

mod model;
mod solution;

pub type Id = String;
pub type Index = usize;

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
