use core::panic;

use crate::model::Model;
use crate::operator::{Operator, Operators};
use crate::random::Random;
use crate::solution::Solution;

#[derive(Default)]
pub struct Solver {
    model: Model,
    operators: Operators,
    options: SolverOptions,
    solution: Option<Solution>,
    random: Random,
    pub iteration_count: usize,
}

impl Solver {
    #[must_use]
    pub fn model(&self) -> &Model {
        &self.model
    }

    #[must_use]
    pub fn operators(&self) -> &Operators {
        &self.operators
    }

    #[must_use]
    pub fn options(&self) -> &SolverOptions {
        &self.options
    }

    #[must_use]
    pub fn solution(&self) -> Option<&Solution> {
        self.solution.as_ref()
    }

    #[must_use]
    pub fn solve(mut self) -> Option<Solution> {
        while self.iteration_count < self.options.max_iterations {
            self.execute_operators();
            self.increment_iteration();
        }
        self.solution
    }

    fn increment_iteration(&mut self) {
        self.iteration_count += 1;
    }

    fn execute_operators(&mut self) {
        let mut solution = self.solution.take().unwrap_or_default();
        for op in self.operators.iter() {
            if !self.random.chance((op.chance(), 1.0)) {
                continue;
            }
            solution = solution
                .plan(&op.execute(&self.model, &solution, &mut self.random))
                .best(solution);
        }
        self.solution = Some(solution);
    }
}

#[derive(Default)]
pub struct SolverBuilder {
    solver: Solver,
}

impl SolverBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn operator<Op: Operator + 'static>(mut self, operator: Op) -> Self {
        self.solver.operators.push(Box::new(operator));
        self
    }

    #[must_use]
    pub fn options(mut self, options: SolverOptions) -> Self {
        self.solver.options = options;
        self
    }

    #[must_use]
    pub fn model(mut self, model: Model) -> SolverBuilder {
        self.solver.model = model;
        self
    }

    #[must_use]
    pub fn plan(mut self, solution: Solution) -> Self {
        self.solver.solution = Some(solution);
        self
    }

    #[must_use]
    pub fn build(self) -> Solver {
        self.solver
    }
}

pub struct SolverOptions {
    max_iterations: usize,
}

impl SolverOptions {
    #[must_use]
    pub fn new(max_iterations: usize) -> Self {
        SolverOptions { max_iterations }
    }
}

impl Default for SolverOptions {
    fn default() -> Self {
        SolverOptions {
            max_iterations: 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::{
            DistanceExpression, ModelBuilder, UnplannedObjective, VehicleCapacityConstraint,
            VehicleCompatibilityConstraint,
        },
        operator::{DestroyOperator, OperatorParameters, RepairOperator, ResetOperator},
    };

    use super::*;

    #[test]
    fn test_solver() {
        let options = SolverOptions::new(10);

        let model = ModelBuilder::new()
            .objective(UnplannedObjective)
            .constraint(VehicleCapacityConstraint::MaxVolume)
            .constraint(VehicleCompatibilityConstraint::Match)
            .expression(DistanceExpression::Meters)
            .build();

        let solver = SolverBuilder::new()
            .operator(RepairOperator::Random(OperatorParameters::new(1.0, 0.5)))
            .operator(RepairOperator::Nearest(OperatorParameters::new(1.0, 0.5)))
            .operator(DestroyOperator::Random(OperatorParameters::new(2.0, 0.3)))
            .operator(DestroyOperator::Nearest(OperatorParameters::new(2.0, 0.3)))
            .operator(ResetOperator::Partial(OperatorParameters::new(3.0, 0.2)))
            .operator(ResetOperator::Full(OperatorParameters::new(3.0, 0.2)))
            .options(options)
            .model(model)
            .build();

        assert_eq!(solver.options.max_iterations, 10);
        assert_eq!(solver.operators().len(), 6);
        assert_eq!(solver.iteration_count, 0);
        assert!(solver.solution.is_none());
        assert_eq!(solver.model.objectives().len(), 1);
        assert_eq!(solver.model.constraints().len(), 2);
    }

    #[test]
    fn test_seeded_random() {
        let mut rng1 = Random::seed(42);
        let mut rng2 = Random::seed(42);

        for _ in 0..100 {
            assert_eq!(rng1.u32(), rng2.u32());
            assert_eq!(rng1.f64(), rng2.f64());
            assert_eq!(rng1.range_u32(0, 100), rng2.range_u32(0, 100));
            assert_eq!(rng1.range_f64(0.0, 1.0), rng2.range_f64(0.0, 1.0));
        }
    }
}
