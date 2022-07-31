/// `ga` is designed to solve both constrained and unconstrained problems by encoding solution traits as
/// *genes*, solution states as *individuals*, and solution groups as *populations*. Genetic algorithms improve on populations
/// iteratively (referred to as *generations*) via reproduction and scoring an individual's *fitness*.
pub mod config;
pub mod individual;
pub mod model;
pub mod population;
pub mod solver;
