use crate::solution::{Plan, Solution};

pub trait Objective {
    /// Name of the objective.
    fn name(&self) -> String;
    /// Computes the value of the objective for the given plan.
    fn compute(&self, model: &Model, solution: &Solution, plan: &Plan) -> f64;
}

pub trait Constraint {
    /// Name of the constraint.
    fn name(&self) -> String;
    /// Checks if the plan violates the constraint.
    fn is_feasible(&self, model: &Model, solution: &Solution, plan: &Plan) -> bool;
    /// Determines when to apply the constraint.
    fn when(&self) -> usize;
    /// Indicates if the constraint is temporal.
    fn is_temporal(&self) -> bool {
        false
    }
}

pub trait Expression {
    /// Name of the expression.
    fn name(&self) -> String;
    /// Computes the value of the expression for the given plan.
    fn compute(&self, model: &Model, solution: &Solution, plan: &Plan) -> f64;
}

#[derive(Default)]
pub struct Model {
    data: ModelData,
    objectives: Objectives,
    constraints: Constraints,
    expressions: Expressions,
}

impl Model {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn stops(&self) -> &Vec<Stop> {
        &self.data.stops.0
    }

    #[must_use]
    pub fn vehicles(&self) -> &Vec<Vehicle> {
        &self.data.vehicles.0
    }

    #[must_use]
    pub fn distance_matrix(&self) -> Option<&DistanceMatrix> {
        self.data.distance_matrix.as_ref()
    }

    #[must_use]
    pub fn objectives(&self) -> &Vec<Box<dyn Objective>> {
        &self.objectives.0
    }

    #[must_use]
    pub fn constraints(&self) -> &Vec<Box<dyn Constraint>> {
        &self.constraints.0
    }

    #[must_use]
    pub fn expressions(&self) -> &Vec<Box<dyn Expression>> {
        &self.expressions.0
    }
}

#[derive(Default)]
struct ModelData {
    stops: Stops,
    vehicles: Vehicles,
    distance_matrix: Option<DistanceMatrix>,
}

#[derive(Default)]
struct Stops(Vec<Stop>);

impl Stops {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn push(&mut self, stop: Stop) {
        self.0.push(stop);
    }
}

#[derive(Default)]
struct Vehicles(Vec<Vehicle>);

impl Vehicles {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn push(&mut self, vehicle: Vehicle) {
        self.0.push(vehicle);
    }
}

#[derive(Default)]
struct Objectives(Vec<Box<dyn Objective>>);
impl Objectives {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, objective: Box<dyn Objective>) {
        self.0.push(objective);
    }
}

#[derive(Default)]
struct Constraints(Vec<Box<dyn Constraint>>);
impl Constraints {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, constraint: Box<dyn Constraint>) {
        self.0.push(constraint);
    }
}

#[derive(Default)]
struct Expressions(Vec<Box<dyn Expression>>);
impl Expressions {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, expression: Box<dyn Expression>) {
        self.0.push(expression);
    }
}

pub struct ModelBuilder {
    data: ModelData,
    objectives: Objectives,
    constraints: Constraints,
    expressions: Expressions,
}

impl Default for ModelBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: ModelData::default(),
            objectives: Objectives::default(),
            constraints: Constraints::default(),
            expressions: Expressions::default(),
        }
    }

    #[must_use]
    pub fn stop(mut self, stop: Stop) -> Self {
        self.data.stops.push(stop);
        self
    }

    #[must_use]
    pub fn vehicle(mut self, vehicle: Vehicle) -> Self {
        self.data.vehicles.push(vehicle);
        self
    }

    #[must_use]
    pub fn distance_matrix(mut self, matrix: DistanceMatrix) -> Self {
        self.data.distance_matrix = Some(matrix);
        self
    }

    #[must_use]
    pub fn build(self) -> Model {
        Model {
            data: self.data,
            objectives: self.objectives,
            constraints: self.constraints,
            expressions: self.expressions,
        }
    }

    #[must_use]
    pub fn objective<O: Objective + 'static>(mut self, objective: O) -> Self {
        self.objectives.push(Box::new(objective));
        self
    }

    #[must_use]
    pub fn constraint<C: Constraint + 'static>(mut self, constraint: C) -> Self {
        self.constraints.push(Box::new(constraint));
        self
    }

    #[must_use]
    pub fn expression<E: Expression + 'static>(mut self, expression: E) -> Self {
        self.expressions.push(Box::new(expression));
        self
    }
}

pub struct UnplannedObjective;

impl Objective for UnplannedObjective {
    fn name(&self) -> String {
        String::from("Unplanned Objective")
    }

    fn compute(&self, _model: &Model, _solution: &Solution, _plan: &Plan) -> f64 {
        0.0
    }
}

pub enum VehicleConstraint {
    Capacity,
    Compatibility,
}

impl Constraint for VehicleConstraint {
    fn name(&self) -> String {
        match self {
            VehicleConstraint::Capacity => String::from("Vehicle Capacity Constraint"),
            VehicleConstraint::Compatibility => String::from("Vehicle Compatibility Constraint"),
        }
    }

    fn is_feasible(&self, _model: &Model, _solution: &Solution, _plan: &Plan) -> bool {
        true
    }

    fn when(&self) -> usize {
        0
    }
}

pub enum DistanceExpression {
    Meters,
}

impl Expression for DistanceExpression {
    fn name(&self) -> String {
        match self {
            DistanceExpression::Meters => String::from("Distance Expression (Meters)"),
        }
    }

    fn compute(&self, _model: &Model, _solution: &Solution, _plan: &Plan) -> f64 {
        match self {
            DistanceExpression::Meters => 0.0,
        }
    }
}

pub struct Stop {
    id: usize,
    location: Location,
    quantities: Vec<f64>,
    compatibility_attributes: Option<Vec<CompatibilityAttribute>>,
}

impl Stop {
    pub fn new(id: usize, location: Location, quantities: Vec<f64>) -> Self {
        Stop {
            id,
            location,
            quantities,
            compatibility_attributes: None,
        }
    }
}

pub struct Vehicle {
    id: usize,
    capacity: Vec<f64>,
    start_location: Option<Location>,
    end_location: Option<Location>,
    compatibility_attributes: Option<Vec<CompatibilityAttribute>>,
}

impl Vehicle {
    pub fn new(id: usize, capacity: Vec<f64>) -> Self {
        Vehicle {
            id,
            capacity,
            start_location: None,
            end_location: None,
            compatibility_attributes: None,
        }
    }
}

pub struct DistanceMatrix {
    matrix: Vec<Vec<f64>>,
}

impl DistanceMatrix {
    pub fn new(matrix: Vec<Vec<f64>>) -> Self {
        DistanceMatrix { matrix }
    }
}

pub struct Location {
    id: usize,
    latitude: f64,
    longitude: f64,
}

impl Location {
    pub fn new(id: usize, latitude: f64, longitude: f64) -> Self {
        Location {
            id,
            latitude,
            longitude,
        }
    }
}

pub struct CompatibilityAttribute {
    key: usize,
    value: String,
}

impl CompatibilityAttribute {
    pub fn new(key: usize, value: String) -> Self {
        CompatibilityAttribute { key, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestObjective;
    impl Objective for TestObjective {
        fn name(&self) -> String {
            String::from("Test Objective")
        }

        fn compute(&self, _model: &Model, _solution: &Solution, _plan: &Plan) -> f64 {
            0.0
        }
    }

    struct TestConstraint;
    impl Constraint for TestConstraint {
        fn name(&self) -> String {
            String::from("Test Constraint")
        }

        fn is_feasible(&self, _model: &Model, _solution: &Solution, _plan: &Plan) -> bool {
            true
        }

        fn when(&self) -> usize {
            0
        }
    }

    struct TestExpression;
    impl Expression for TestExpression {
        fn name(&self) -> String {
            String::from("Test Expression")
        }

        fn compute(&self, _model: &Model, _solution: &Solution, _plan: &Plan) -> f64 {
            0.0
        }
    }

    #[test]
    fn test_model() {
        let stop = Stop::new(1, Location::new(1, 10.0, 20.0), vec![5.0]);
        let vehicle = Vehicle::new(1, vec![10.0]);
        let distance_matrix = DistanceMatrix::new(vec![vec![0.0, 1.0], vec![1.0, 0.0]]);

        let model = ModelBuilder::new()
            .stop(stop)
            .vehicle(vehicle)
            .distance_matrix(distance_matrix)
            .objective(UnplannedObjective)
            .objective(TestObjective)
            .constraint(VehicleConstraint::Capacity)
            .constraint(VehicleConstraint::Compatibility)
            .constraint(TestConstraint)
            .expression(DistanceExpression::Meters)
            .expression(TestExpression)
            .build();

        assert_eq!(model.stops().len(), 1);
        assert_eq!(model.vehicles().len(), 1);
        assert!(model.distance_matrix().is_some());

        assert_eq!(model.objectives().len(), 2);
        assert_eq!(model.constraints().len(), 3);
        assert_eq!(model.expressions().len(), 2);

        assert_eq!(model.objectives()[0].name(), "Unplanned Objective");
        assert_eq!(model.constraints()[0].name(), "Vehicle Capacity Constraint");
        assert_eq!(
            model.constraints()[1].name(),
            "Vehicle Compatibility Constraint"
        );

        assert_eq!(model.objectives()[1].name(), "Test Objective");
        assert_eq!(model.constraints()[2].name(), "Test Constraint");
        assert_eq!(
            model.expressions()[0].name(),
            "Distance Expression (Meters)"
        );
    }
}
