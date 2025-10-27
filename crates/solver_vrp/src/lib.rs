//! # `solver-vrp`
//!
//! A vehicle routing solver library.
//!
//! ! WARNING: Some APIs like the Plan API are still unimplemented and subject to change.
//!
//! This library provides functionalities to solve various vehicle routing problems (VRP), including:
//!
//! - Pickup and Delivery Problem (PDP)
//!
//! ```rust,ignore
//! use solver_vrp::model::{Constraint, Expression, Model, ModelBuilder, Objective};
//! use solver_vrp::operator::Operator;
//! use solver_vrp::solution::{Plan, Solution};
//! use solver_vrp::solver::{Solver, SolverBuilder, SolverOptions};
//! use solver_vrp::random::Random;
//!
//! // Implement a custom objective to add optimized features like linehaul costs to the model.
//! struct ZeroObjective {
//!     zero: f64,
//! }
//!
//! impl Objective for ZeroObjective {
//!     fn name(&self) -> String {
//!         String::from("My Zero Objective")
//!     }
//!
//!     // Computes the value of the objective for the given plan.
//!     fn compute(&self, _model: &Model, _solution: &Solution, _plan: &Plan) -> f64 {
//!         self.zero
//!     }
//! }
//!
//! // Implement a custom constraint to enforce unique business rules in the model.
//! struct MyVehicleCapacities(vec![f64; 2]);
//!
//! impl Constraint for MyVehicleCapacities {
//!     fn name(&self) -> String {
//!         String::from("My Vehicle Capacities")
//!     }
//!
//!     // Returns true if the plan is feasible.
//!     fn is_feasible(&self, plan: &Plan) -> bool {
//!         let i = 1;
//!         self.0.get(i)
//!             .zip(plan.route().changes().last())
//!             .and_then(|(max, change)| change.required_capacity().get(i).map(|d| d <= max))
//!             .unwrap_or(true)
//!     }
//! }
//!
//! // Add new operators to refine the solver's search and heuristic capabilities.
//! struct SimpleOperator {};
//!
//! impl Operator for SimpleOperator {
//!     fn name(&self) -> String {
//!         String::from("Simple Operator")
//!     }
//!
//!     // Returns the new plan after executing the operator.
//!     fn execute(&self, _model: &Model, _solution: &Solution, _random: &mut Random) -> Plan {
//!         return Plan::new();
//!     }
//! }
//!
//! fn run() {
//!     // Build the model with custom components.
//!     let model = ModelBuilder::new()
//!         .objective(ZeroObjective { zero: 0.0 })
//!         .constraint(MyVehicleCapacities(vec![26.0, 40_000.0]))
//!         .build();
//!
//!     // Define options for the solver.
//!     let max_iterations = 1000;
//!     let options = SolverOptions::new(max_iterations);
//!     // Start with an initial solution.
//!     let initial_solution = Solution::new();
//!     // Build the solver with the custom operator.
//!     let solver = SolverBuilder::new()
//!         .options(options)
//!         .model(model)
//!         .solution(initial_solution)
//!         .operator(SimpleOperator {})
//!         .build();
//!
//!     let best_solution = solver.solve();
//! }
//! ```
//!
//! The solver uses three main operators to iteratively improve the solution:
//!
//! - Repair
//! - Destroy
//! - Reset
//!
//! # `Repair`
//!
//! The repair operator is responsible for reinserting plan units (planned stops) into the solution.
//!
//! # `Destroy`
//!
//! The destroy operator is responsible for removing plan units (planned stops) from the solution.
//!
//! # `Reset`
//!
//! The reset operator helps the solver backtrack by resetting parts of or whole solutions.
//!
//! # `Solver`
//!
//! The `Solver` is designed to implement specific defaults that can be extended from. Override internal
//! options and express your `Model` and `Solver` with custom objectives, constraints, and operators. By
//! default, this is a ALNS (Adaptive Large Neighborhood Search) solver that uses multiple strategies to
//! explore the solution space.
//!
//! # `Model`
//!
//! The `Model` struct represents the vehicle routing problem instance to be solved. It contains all
//! of the input data as well as the definitions for the model. Inputs include stops, vehicles, and
//! a distance matrix.
//!
//! # `Solution`
//!
//! The `Solution` struct represents a solution to the vehicle routing problem. It contains the vehicles
//! with assigned plan units (planned stops). It also contains unassigned plan units.
//!
//! # `Vehicle`
//!
//! The `Solver` will assign and unassign routed stops.
//!
//! # `Stop`
//!
//! A stop represents a location that must be visited as part of a plan unit. Stops have specifications
//! like location coordinates, quantities, and compatibility attributes.
//!
//! # `DistanceMatrix`
//!
//! A distance matrix provides the distances between each stop in the model.
//!
//! # `Objective`
//!
//! The `Solver` uses implementations of `Objective` to evaluate and optimize solutions. The model is a
//! weighted sum of all its objectives.
//!
//! # `Constraint`
//!
//! Constraints define the rules for each solution plan.
//!
//! # `Expression`
//!
//! Every model implements some number of expressions that are used for internal calculations.

mod model;
mod operator;
mod random;
mod solution;
mod solver;
