use crate::solution::Plan;

#[derive(Default)]
pub struct Objectives(Vec<Box<dyn Objective>>);

pub trait Objective {
    /// Name of the objective.
    fn name(&self) -> String;
    /// Computes the value of the objective for the given plan.
    fn compute(&self, plan: &Plan) -> f64;
}

impl Objectives {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&dyn Objective> {
        self.0.get(index).map(AsRef::as_ref)
    }

    pub fn first(&self) -> Option<&dyn Objective> {
        self.0.first().map(AsRef::as_ref)
    }

    pub fn push(&mut self, objective: Box<dyn Objective>) {
        self.0.push(objective);
    }
}

#[derive(Default)]
pub struct UnplannedObjective;

impl Objective for UnplannedObjective {
    fn name(&self) -> String {
        String::from("unplanned")
    }

    fn compute(&self, _plan: &Plan) -> f64 {
        todo!()
    }
}
