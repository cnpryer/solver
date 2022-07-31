/// `ga` is designed to solve both constrained and unconstrained problems by encoding solution traits as
/// *genes*, solution states as *individuals*, and solution groups as *populations*. Genetic algorithms improve on populations
/// iteratively (referred to as *generations*) via reproduction and scoring an individual's *fitness*.
mod config;
mod individual;
mod model;
mod population;

const SOLVER_NAME: &str = "GeneticAlgorithm";

pub struct Solver<'a> {
    name: &'a str,
    model: model::Model<'a>,
    config: Option<config::Config>,
}

impl Solver<'_> {
    fn new(model: model::Model<'_>, config: Option<config::Config>) -> Solver {
        Solver {
            name: SOLVER_NAME,
            model,
            config,
        }
    }
}

// TODO
fn run(solver: &Solver) -> () {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_fitness_fn(individual: &individual::Individual) -> i32 {
        // TODO
        individual.get_fitness().clone()
    }

    #[test]
    fn test_solver() {
        let model = model::Model::new(
            population::Population::new(vec![
                individual::Individual::new(vec![1, 2, 3]),
                individual::Individual::new(vec![1, 2, 3]),
            ]),
            &mock_fitness_fn,
        );
        let test_solver = Solver::new(model, None);

        // TODO
        run(&test_solver);

        assert_eq!(test_solver.name, SOLVER_NAME);
    }
}
