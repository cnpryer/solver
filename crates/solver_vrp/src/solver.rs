use crate::model::Model;
use crate::solution::Solution;
use rand::prelude::{Rng, SeedableRng, StdRng};

pub trait Operator {
    /// Name of the operator.
    fn name(&self) -> String;
    /// Executes the operator to generate a new solution.
    fn execute(&self, model: &Model, solution: &Solution) -> Option<Solution>;
}

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
}

#[derive(Default)]
pub struct Operators(Vec<Box<dyn Operator>>);

impl Operators {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn first(&self) -> Option<&dyn Operator> {
        self.0.first().map(AsRef::as_ref)
    }

    pub fn get(&self, index: usize) -> Option<&dyn Operator> {
        self.0.get(index).map(AsRef::as_ref)
    }

    pub fn push(&mut self, operator: Box<dyn Operator>) {
        self.0.push(operator);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Box<dyn Operator>> {
        self.0.iter()
    }
}

impl Solver {
    #[must_use]
    pub fn solve(mut self) -> Option<Solution> {
        while self.options.max_iterations > self.iteration_count {
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
            if let Some(s) = op.execute(&self.model, &solution) {
                solution = s.best(solution);
            }
        }
        self.solution = Some(solution);
    }
}

#[derive(Default)]
pub struct SolverBuilder {
    operators: Operators,
    options: SolverOptions,
}

impl SolverBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn operator<Op: Operator + 'static>(mut self, operator: Op) -> Self {
        self.operators.push(Box::new(operator));
        self
    }

    #[must_use]
    pub fn options(mut self, options: SolverOptions) -> Self {
        self.options = options;
        self
    }

    #[must_use]
    pub fn model(self, model: Model) -> SolverBuilderWithModel {
        SolverBuilderWithModel {
            solver: Solver {
                model,
                operators: self.operators,
                options: self.options,
                solution: None,
                random: Random::new(),
                iteration_count: 0,
            },
        }
    }
}

pub struct SolverBuilderWithModel {
    solver: Solver,
}

impl SolverBuilderWithModel {
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
    pub fn build(self) -> Solver {
        self.solver
    }

    pub fn plan(mut self, solution: Solution) -> Self {
        self.solver.solution = Some(solution);
        self
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

pub enum RepairOperator {
    Random(OperatorParameters),
    Nearest(OperatorParameters),
}

impl Operator for RepairOperator {
    fn name(&self) -> String {
        match self {
            Self::Random(_) => "Random Repair Operator".to_string(),
            Self::Nearest(_) => "Nearest Repair Operator".to_string(),
        }
    }

    fn execute(&self, _model: &Model, _solution: &Solution) -> Option<Solution> {
        None
    }
}

pub enum DestroyOperator {
    Random(OperatorParameters),
    Nearest(OperatorParameters),
}

impl Operator for DestroyOperator {
    fn name(&self) -> String {
        match self {
            Self::Random(_) => "Random Destroy Operator".to_string(),
            Self::Nearest(_) => "Nearest Destroy Operator".to_string(),
        }
    }

    fn execute(&self, _model: &Model, _solution: &Solution) -> Option<Solution> {
        None
    }
}

pub enum ResetOperator {
    Full(OperatorParameters),
    Partial(OperatorParameters),
}

impl Operator for ResetOperator {
    fn name(&self) -> String {
        match self {
            Self::Full(_) => "Full Reset Operator".to_string(),
            Self::Partial(_) => "Partial Reset Operator".to_string(),
        }
    }

    fn execute(&self, _model: &Model, _solution: &Solution) -> Option<Solution> {
        None
    }
}

pub struct OperatorParameters {
    value: f64,
    chance: f64,
}

impl OperatorParameters {
    #[must_use]
    pub fn new(value: f64, chance: f64) -> Self {
        Self { value, chance }
    }
}

struct Random {
    rng: StdRng,
}

impl Random {
    fn new() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    fn seed(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    fn u32(&mut self) -> u32 {
        self.rng.random()
    }

    fn f64(&mut self) -> f64 {
        self.rng.random()
    }

    fn range_u32(&mut self, low: u32, high: u32) -> u32 {
        self.rng.random_range(low..high)
    }

    fn range_f64(&mut self, low: f64, high: f64) -> f64 {
        self.rng.random_range(low..high)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ModelBuilder,
        model::{
            DistanceExpression, UnplannedObjective, VehicleCapacityConstraint,
            VehicleCompatibilityConstraint,
        },
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
