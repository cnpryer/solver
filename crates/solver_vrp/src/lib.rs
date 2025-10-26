//! # `solver-vrp`
//!
//! A vehicle routing solver library.
//!
//! This library provides functionalities to solve various vehicle routing problems (VRP), including
//!
//! - Pickup and Delivery Problem (PDP)
//!
//! ```
//! use solver_vrp::{Constraint, Expression, Model, ModelBuilder, Objective, Operator, Plan, Solution, SolverBuilder, SolverOptions};
//!
//! // Implement a custom objective to add optimized features like linehaul costs to the model.
//! struct CustomObjective;
//!
//! impl Objective for CustomObjective {
//!     fn name(&self) -> String {
//!         String::from("Custom Objective")
//!     }
//!
//!     // Returns the computed value of the objective for the given plan.
//!     fn compute(&self, _model: &Model, _solution: &Solution, _plan: &Plan) -> f64 {
//!         0.0
//!     }
//! }
//!
//! // Implement a custom constraint to enforce unique business rules in the model.
//! struct CustomConstraint;
//!
//! impl Constraint for CustomConstraint {
//!     fn name(&self) -> String {
//!         String::from("Custom Objective")
//!     }
//!
//!     // Returns true if the plan is feasible.
//!     fn is_feasible(&self, _model: &Model, _solution: &Solution, _plan: &Plan) -> bool {
//!        true
//!     }
//!
//!     // Returns the priority of the constraint.
//!     fn when(&self) -> usize {
//!         0
//!     }
//!
//!     // Returns true if the constraint is temporal.
//!     fn is_temporal(&self) -> bool {
//!         false
//!     }
//! }
//!
//! // Implement a custom expression to add new calculations to the model.
//! struct CustomExpression;
//!
//! impl Expression for CustomExpression {
//!     fn name(&self) -> String {
//!         String::from("Custom Expression")
//!     }
//!     
//!     // Returns the computed value for the expression.
//!     fn compute(&self, _model: &Model, _solution: &Solution, _plan: &Plan) -> f64 {
//!         0.0
//!     }
//! }
//!
//! // Add new operators to refine the solver's search and heuristic capabilities.
//! struct CustomOperator;
//!
//! impl Operator for CustomOperator {
//!     fn name(&self) -> String {
//!        String::from("Custom Operator")
//!     }
//!
//!     // Returns the new solution after executing the operator.
//!     fn execute(&self, _model: &Model, _solution: &Solution) -> Option<Solution> {
//!         None
//!     }
//! }
//!
//! fn run() {
//!     // Build the model with custom components.
//!     let model = ModelBuilder::new()
//!         .objective(CustomObjective)
//!         .constraint(CustomConstraint)
//!         .expression(CustomExpression)
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
//!         .operator(CustomOperator)
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
//! options and express your `Model` and `Solver` with custom objectives, constraints, expressions,
//! and operators. By default, this is a ALNS (Adaptive Large Neighborhood Search) solver that uses multiple
//! strategies to explore the solution space.
//!
//! # `Model`
//!
//! The `Model` struct represents the vehicle routing problem instance to be solved. It contains all
//! of the input data as well as the definitions for the model. Inputs include stops, vehicles, and
//! a distance matrix. Objectives, constraints, and expressions are used to define and extend the model.
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
mod solution;
mod solver;

pub use model::{Constraint, Expression, Model, ModelBuilder, Objective};
pub use solution::{Plan, Solution};
pub use solver::{Operator, Solver, SolverBuilder, SolverOptions};
