use crate::solution::Plan;

pub trait Constraint {
    /// Name of the constraint.
    fn name(&self) -> String;
    /// Checks if the plan violates the constraint.
    fn is_feasible(&self, plan: &Plan) -> bool;
    /// Indicates if the constraint is temporal.
    fn is_temporal(&self) -> bool {
        false
    }
}

#[derive(Default)]
pub struct Constraints(Vec<Box<dyn Constraint>>);

impl Constraints {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&dyn Constraint> {
        self.0.get(index).map(AsRef::as_ref)
    }

    pub fn first(&self) -> Option<&dyn Constraint> {
        self.0.first().map(AsRef::as_ref)
    }

    pub fn push(&mut self, constraint: Box<dyn Constraint>) {
        self.0.push(constraint);
    }
}

#[derive(Default)]
pub struct VehicleCapacityConstraint {}

impl Constraint for VehicleCapacityConstraint {
    fn name(&self) -> String {
        String::from("vehicle_capacity")
    }

    fn is_feasible(&self, plan: &Plan) -> bool {
        todo!()
    }
}

pub struct VehicleCompatibilityConstraint {
    compatible: StopCompatibilities,
}

impl VehicleCompatibilityConstraint {
    pub fn new(compatible: StopCompatibilities) -> Self {
        Self { compatible }
    }
}

impl Default for VehicleCompatibilityConstraint {
    fn default() -> Self {
        Self {
            compatible: StopCompatibilities(Vec::new()),
        }
    }
}

impl Constraint for VehicleCompatibilityConstraint {
    fn name(&self) -> String {
        String::from("vehicle_compatibility")
    }

    fn is_feasible(&self, plan: &Plan) -> bool {
        self.compatible
            .is_compatible(plan.stop().id, plan.vehicle().id)
    }
}

pub struct StopCompatibilities(Vec<Vec<bool>>);

impl StopCompatibilities {
    pub fn is_compatible(&self, stop_index: usize, vehicle_index: usize) -> bool {
        self.0
            .get(stop_index)
            .is_some_and(|v| v.get(vehicle_index).copied().unwrap_or(true))
    }
}
