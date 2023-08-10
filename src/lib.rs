//! Solver: <small>A generic solver library.</small>

/// `ga` (genetic algorithms) is designed to solve both constrained and unconstrained problems by encoding solution traits as
/// *genes*, solution states as *individuals*, and solution groups as *populations*. Genetic algorithms improve on populations
/// iteratively (referred to as *generations*) via reproduction and scoring an individual's *fitness*.
pub mod ga;

#[cfg(test)]
mod tests {
    use crate::ga::{
        config::Config, individual::Individual, model::Model, population::Population,
        solver::Solver,
    };

    fn mock_fitness_fn(individual: &Individual) -> u32 {
        // TODO
        *individual.get_fitness()
    }

    #[test]
    fn solve_schedule() {
        let model = Model::new(
            Population::new(0, vec![Individual::new(vec![1, 2, 3], u32::MIN)]),
            &mock_fitness_fn,
            Config::default(),
        );

        let solver = Solver::new(model);

        // TODO
        solver.solve();
    }
}
