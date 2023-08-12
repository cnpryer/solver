use crate::ga::model::Model;

const SOLVER_NAME: &str = "GeneticAlgorithm";

pub struct Solver<'a> {
    name: &'a str,
    model: Model<'a>,
}

impl Solver<'_> {
    pub fn new(model: Model<'_>) -> Solver {
        Solver {
            name: SOLVER_NAME,
            model,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn get_model(&self) -> &Model {
        &self.model
    }

    pub fn solve(&self) {}
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::ga::{config::Config, individual::Individual, *},
    };

    fn mock_fitness_fn(individual: &Individual) -> u32 {
        // TODO
        *individual.get_fitness()
    }

    #[test]
    fn test_solver() {
        let model = model::Model::new(
            population::Population::new(
                0,
                vec![
                    Individual::new(vec![1, 2, 3], u32::MIN),
                    Individual::new(vec![1, 2, 3], u32::MIN),
                ],
            ),
            &mock_fitness_fn,
            Config::default(),
        );
        let test_solver = Solver::new(model);

        assert_eq!(test_solver.name, SOLVER_NAME);
    }
}
