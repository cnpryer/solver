/// `ga` is designed to solve both constrained and unconstrained problems by encoding solution traits as
/// *genes*, solution states as *individuals*, and solution groups as *populations*. Genetic algorithms improve on populations
/// iteratively (referred to as *generations*) via reproduction and scoring an individual's *fitness*.
use self::config::Config;
use self::model::Model;

pub mod config;
pub mod individual;
pub mod model;
pub mod population;

const SOLVER_NAME: &str = "GeneticAlgorithm";

pub struct Solver<'a> {
    name: &'a str,
    model: Model<'a>,
    config: Config,
}

impl Solver<'_> {
    pub fn new(model: model::Model<'_>, config: Config) -> Solver {
        Solver {
            name: SOLVER_NAME,
            model,
            config,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_model(&self) -> &Model {
        &self.model
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::{individual::Individual, *};

    fn mock_fitness_fn(individual: &Individual) -> i32 {
        // TODO
        individual.get_fitness().clone()
    }

    #[test]
    fn test_solver() {
        let model = model::Model::new(
            population::Population::new(
                0,
                vec![
                    Individual::new(vec![1, 2, 3], i32::MIN),
                    Individual::new(vec![1, 2, 3], i32::MIN),
                ],
            ),
            &mock_fitness_fn,
        );
        let test_solver = Solver::new(model, Config::default());

        // TODO
        // run(&test_solver);

        assert_eq!(test_solver.name, SOLVER_NAME);
    }
}
