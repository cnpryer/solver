//! # `solver-vrp`
//!
//! A vehicle routing solver library.
//!
//! This library provides functionalities to solve various vehicle routing problems (VRP), including
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
//! impl Objective for ZeroObjective {
//!     fn name(&self) -> String {
//!         String::from("Zero Objective")
//!     }
//!
//!     // Returns the computed value of the objective for the given plan.
//!     fn compute(&self, _model: &Model, _solution: &Solution, _plan: &Plan) -> f64 {
//!         0.0
//!     }
//! }
//!
//! // Implement a custom constraint to enforce unique business rules in the model.
//! struct MaxVehicleWeight((f64, f64));
//!
//! impl Constraint for MaxVehicleWeight {
//!     fn name(&self) -> String {
//!         String::from("Max Vehicle Weight")
//!     }
//!
//!     // Returns true if the plan is feasible.
//!     fn is_feasible(&self, plan: &Plan) -> bool {
//!         plan.route()
//!             .changes()
//!             .last()
//!             .map_or(true, |change| {
//!                 change.capacity.utilization(1) <= self.0.1
//!             });
//!     }
//! }
//!
//! // Add new operators to refine the solver's search and heuristic capabilities.
//! #![derive(Default)]
//! struct RejectEverything {
//!     parameters: OperatorParameters,
//! };
//!
//! impl Operator for CustomOperator {
//!     fn name(&self) -> String {
//!        String::from("Custom Operator")
//!     }
//!
//!     // Returns the new solution after executing the operator.
//!     fn execute(&self, _model: &Model, _solution: &Solution, _random: &mut Random) -> Plan {
//!         Plan::default()
//!     }
//! }
//!
//! fn run() {
//!     // Build the model with custom components.
//!     let model = ModelBuilder::new()
//!         .objective(CustomObjective::default())
//!         .constraint(CustomConstraint((26.0, 40_000.0)))
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
//!         .plan(initial_solution)
//!         .operator(CustomOperator {})
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
