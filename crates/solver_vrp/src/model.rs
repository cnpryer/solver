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
    pub fn data(&self) -> &ModelData {
        &self.data
    }

    #[must_use]
    pub fn stops(&self) -> &Stops {
        &self.data.stops
    }

    #[must_use]
    pub fn vehicles(&self) -> &Vehicles {
        &self.data.vehicles
    }

    #[must_use]
    pub fn distance_matrix(&self) -> Option<&DistanceMatrix> {
        self.data.distance_matrix.as_ref()
    }

    #[must_use]
    pub fn objectives(&self) -> &Objectives {
        &self.objectives
    }

    #[must_use]
    pub fn constraints(&self) -> &Constraints {
        &self.constraints
    }

    #[must_use]
    pub fn expressions(&self) -> &Expressions {
        &self.expressions
    }
}

#[derive(Default)]
pub struct ModelData {
    stops: Stops,
    vehicles: Vehicles,
    distance_matrix: Option<DistanceMatrix>,
    graph: DirectedAcyclicGraph,
}

#[derive(Default)]
pub struct Stops(Vec<Stop>);

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
pub struct Vehicles(Vec<Vehicle>);

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
pub struct Objectives(Vec<Box<dyn Objective>>);

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
pub struct Expressions(Vec<Box<dyn Expression>>);

impl Expressions {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&dyn Expression> {
        self.0.get(index).map(AsRef::as_ref)
    }

    pub fn first(&self) -> Option<&dyn Expression> {
        self.0.first().map(AsRef::as_ref)
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

pub enum VehicleCapacityConstraint {
    MaxWeight,
    MaxVolume,
}

pub enum VehicleCompatibilityConstraint {
    Match,
}

impl Constraint for VehicleCapacityConstraint {
    fn name(&self) -> String {
        match self {
            VehicleCapacityConstraint::MaxWeight => {
                String::from("Vehicle Capacity Constraint (Max Weight)")
            }
            VehicleCapacityConstraint::MaxVolume => {
                String::from("Vehicle Capacity Constraint (Max Volume)")
            }
        }
    }

    fn is_feasible(&self, model: &Model, solution: &Solution, plan: &Plan) -> bool {
        match self {
            VehicleCapacityConstraint::MaxWeight => true,
            VehicleCapacityConstraint::MaxVolume => true,
        }
    }
}

impl Constraint for VehicleCompatibilityConstraint {
    fn name(&self) -> String {
        match self {
            VehicleCompatibilityConstraint::Match => {
                String::from("Vehicle Compatibility Constraint (Match)")
            }
        }
    }

    fn is_feasible(&self, model: &Model, solution: &Solution, plan: &Plan) -> bool {
        match self {
            VehicleCompatibilityConstraint::Match => true,
        }
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

#[derive(Default)]
struct DirectedAcyclicGraph {
    edges: Vec<Vec<usize>>,
    outbound_arcs: Vec<Vec<Arc>>,
    arcs: Vec<Arc>,
}

impl DirectedAcyclicGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(node_count: usize) -> Self {
        Self {
            edges: vec![Vec::new(); node_count],
            outbound_arcs: vec![Vec::new(); node_count],
            arcs: Vec::new(),
        }
    }

    pub fn add_arc(&mut self, from: usize, to: usize) {
        let arc = Arc { from, to };
        self.edges[from].push(to);
        self.outbound_arcs[from].push(arc.clone());
        self.arcs.push(arc);
    }

    pub fn outbound(&self, node: usize) -> &[Arc] {
        &self.outbound_arcs[node]
    }

    pub fn arcs(&self) -> &[Arc] {
        &self.arcs
    }

    pub fn edges(&self) -> &[Vec<usize>] {
        &self.edges
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Arc {
    from: usize,
    to: usize,
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
            .constraint(VehicleCapacityConstraint::MaxVolume)
            .constraint(VehicleCompatibilityConstraint::Match)
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

        assert_eq!(
            model.objectives().first().map(|o| o.name()),
            Some(String::from("Unplanned Objective"))
        );
        assert_eq!(
            model.objectives().get(1).map(|o| o.name()),
            Some(String::from("Test Objective"))
        );
        assert_eq!(
            model.constraints().first().map(|c| c.name()),
            Some(String::from("Vehicle Capacity Constraint (Max Volume)"))
        );
        assert_eq!(
            model.constraints().get(1).map(|c| c.name()),
            Some(String::from("Vehicle Compatibility Constraint (Match)"))
        );
        assert_eq!(
            model.constraints().get(2).map(|c| c.name()),
            Some(String::from("Test Constraint"))
        );
        assert_eq!(
            model.expressions().first().map(|e| e.name()),
            Some(String::from("Distance Expression (Meters)"))
        );
    }

    #[test]
    fn test_graph() {
        let mut graph = DirectedAcyclicGraph::with_capacity(3);
        graph.add_arc(0, 1);
        graph.add_arc(1, 2);
        assert_eq!(graph.edges().len(), 3);
        assert_eq!(graph.edges()[0], vec![1]);
        assert_eq!(graph.edges()[1], vec![2]);
        assert_eq!(graph.edges()[2], vec![]);
        assert_eq!(graph.arcs().len(), 2);
    }
}
