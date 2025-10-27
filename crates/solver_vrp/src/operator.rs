use crate::model::Model;
use crate::random::Random;
use crate::solution::{Plan, Solution};

pub trait Operator {
    /// Name of the operator.
    fn name(&self) -> String;
    /// Executes the operator to generate a new solution.
    fn execute(&self, model: &Model, solution: &Solution, random: &mut Random) -> Plan;
    /// Chance of applying the operator.
    fn chance(&self) -> f64 {
        1.0
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

pub struct RepairOperator {
    pub parameters: OperatorParameters,
}

impl Default for RepairOperator {
    fn default() -> Self {
        Self {
            parameters: OperatorParameters::new(1.0, 1.0),
        }
    }
}

impl Operator for RepairOperator {
    fn name(&self) -> String {
        String::from("repair")
    }

    fn execute(&self, model: &Model, solution: &Solution, random: &mut Random) -> Plan {
        repair_nearest(model, solution, &self.parameters, random)
    }

    fn chance(&self) -> f64 {
        self.parameters.chance_f64
    }
}

pub struct DestroyOperator {
    pub parameters: OperatorParameters,
}

impl Default for DestroyOperator {
    fn default() -> Self {
        Self {
            parameters: OperatorParameters::new(1.0, 1.0),
        }
    }
}

impl Operator for DestroyOperator {
    fn name(&self) -> String {
        String::from("destroy")
    }

    fn execute(&self, model: &Model, solution: &Solution, random: &mut Random) -> Plan {
        destroy_random(model, solution, &self.parameters, random)
    }

    fn chance(&self) -> f64 {
        self.parameters.chance_f64
    }
}

fn repair_random(
    _model: &Model,
    _solution: &Solution,
    _params: &OperatorParameters,
    _random: &mut Random,
) -> Plan {
    todo!()
}

fn repair_nearest(
    _model: &Model,
    _solution: &Solution,
    _params: &OperatorParameters,
    _random: &mut Random,
) -> Plan {
    todo!()
}

fn destroy_random(
    _model: &Model,
    _solution: &Solution,
    _params: &OperatorParameters,
    _random: &mut Random,
) -> Plan {
    todo!()
}

fn destroy_nearest(
    _model: &Model,
    _solution: &Solution,
    _params: &OperatorParameters,
    _random: &mut Random,
) -> Plan {
    todo!()
}

pub struct OperatorParameters {
    pub value: f64,
    pub chance_f64: f64,
}

impl OperatorParameters {
    #[must_use]
    pub fn new(value: f64, chance_f64: f64) -> Self {
        Self { value, chance_f64 }
    }
}
