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

pub enum RepairOperator {
    Random(OperatorParameters),
    Nearest(OperatorParameters),
}

impl Operator for RepairOperator {
    fn name(&self) -> String {
        match self {
            Self::Random(_) => String::from("Repair Operator (Random)"),
            Self::Nearest(_) => String::from("Repair Operator (Nearest)"),
        }
    }

    fn execute(&self, model: &Model, solution: &Solution, random: &mut Random) -> Plan {
        match self {
            Self::Random(p) => repair_random(model, solution, p, random),
            Self::Nearest(p) => repair_nearest(model, solution, p, random),
        }
    }

    fn chance(&self) -> f64 {
        match self {
            RepairOperator::Random(p) | RepairOperator::Nearest(p) => p.chance_f64,
        }
    }
}

pub enum DestroyOperator {
    Random(OperatorParameters),
    Nearest(OperatorParameters),
}

impl Operator for DestroyOperator {
    fn name(&self) -> String {
        match self {
            Self::Random(_) => String::from("Destroy Operator (Random)"),
            Self::Nearest(_) => String::from("Destroy Operator (Nearest)"),
        }
    }

    fn execute(&self, model: &Model, solution: &Solution, random: &mut Random) -> Plan {
        match self {
            Self::Random(p) => destroy_random(model, solution, p, random),
            Self::Nearest(p) => destroy_nearest(model, solution, p, random),
        }
    }

    fn chance(&self) -> f64 {
        match self {
            DestroyOperator::Random(p) | DestroyOperator::Nearest(p) => p.chance_f64,
        }
    }
}

pub enum ResetOperator {
    Full(OperatorParameters),
    Partial(OperatorParameters),
}

impl Operator for ResetOperator {
    fn name(&self) -> String {
        match self {
            Self::Full(_) => String::from("Reset Operator (Full)"),
            Self::Partial(_) => String::from("Reset Operator (Partial)"),
        }
    }

    fn execute(&self, model: &Model, solution: &Solution, random: &mut Random) -> Plan {
        match self {
            Self::Full(p) => reset_full(model, solution, p, random),
            Self::Partial(p) => reset_partial(model, solution, p, random),
        }
    }

    fn chance(&self) -> f64 {
        match self {
            ResetOperator::Full(p) | ResetOperator::Partial(p) => p.chance_f64,
        }
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

fn reset_full(
    _model: &Model,
    _solution: &Solution,
    _params: &OperatorParameters,
    _random: &mut Random,
) -> Plan {
    todo!()
}

fn reset_partial(
    _model: &Model,
    _solution: &Solution,
    _params: &OperatorParameters,
    _random: &mut Random,
) -> Plan {
    todo!()
}

pub struct OperatorParameters {
    value: f64,
    chance_f64: f64,
}

impl OperatorParameters {
    #[must_use]
    pub fn new(value: f64, chance_f64: f64) -> Self {
        Self { value, chance_f64 }
    }
}
