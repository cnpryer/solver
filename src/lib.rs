pub mod ga;

#[cfg(test)]
mod tests {
    use crate::ga::{
        config::Config, individual::Individual, model::Model, population::Population,
        solver::Solver,
    };

    fn mock_fitness_fn(individual: &Individual) -> i32 {
        // TODO
        individual.get_fitness().clone()
    }

    #[test]
    fn solve_schedule() {
        let model = Model::new(
            Population::new(0, vec![Individual::new(vec![1, 2, 3], i32::MIN)]),
            &mock_fitness_fn,
            Config::default(),
        );

        let solver = Solver::new(model);

        // TODO
        solver.solve();
    }
}
