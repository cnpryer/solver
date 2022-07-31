use std::borrow::Borrow;

use rand::{thread_rng, Rng};

use crate::ga::individual::Individual;
use crate::ga::population::Population;

/// A `Model` is a structure that defines the problem to be solved.
pub struct Model<'a> {
    // first generation of individuals
    population: Population,
    // fitness evaluation function that evaluates an Individual
    fitness_fn: &'a dyn Fn(&Individual) -> i32,
}

impl Model<'_> {
    pub fn new<'a>(
        population: Population,
        fitness_fn: &'a dyn Fn(&Individual) -> i32,
    ) -> Model<'a> {
        Model {
            population,
            fitness_fn,
        }
    }

    /// Applies a new fitness score to each individual in the population.
    fn score_population(&mut self) {
        let mut individuals = self.population.get_individuals().clone();

        for i in 0..individuals.len() {
            let score = (self.fitness_fn)(&individuals[i]);
            individuals[i].update_fitness_score(score);
        }

        self.population.update_individuals(individuals);
    }

    /// Selects a subset of the modeled population based on fitness scores and a `selection_rate`.
    fn select_for_reproduction(&mut self, selection_rate: f32) -> Vec<Individual> {
        // Sort the population individuals in descending order based on their fitness scores
        self.population.sort_by_fitness(); // TODO: Avoid this by retaining pre-sorted pop.

        // get top-n individuals using `selection_rate`
        let n = ((self.population.get_individuals().len() as f32) * selection_rate) as usize;
        self.population.get_individuals()[..n].to_vec()
    }

    /// Breed two parents using a `crossover_rate`.
    fn reproduce(
        &self,
        parent_a: Individual,
        parent_b: Individual,
        crossover_rate: f32,
    ) -> Individual {
        // `crossover_rate` is used to slice n-length of `parent_a` and the remaining length of `parent_b`
        let n = ((parent_a.get_genes().len() as f32) * crossover_rate) as usize;
        let mut new_genes = parent_a.get_genes()[0..n].to_vec();
        new_genes.extend(&parent_b.get_genes()[n..]);

        Individual::new(new_genes)
    }

    /// Creates.
    fn mutate_individual(&mut self, individual: &mut Individual, gene_pool: Vec<u16>) {
        // Pull random gene from the `gene_pool`
        let mut rng = thread_rng();
        let i = rng.gen_range(0..gene_pool.len());
        let new_gene = gene_pool[i];

        // Update a random gene from the individual
        let n = individual.get_genes().len();
        let i = rng.gen_range(0..n);
        individual.update_gene(i, new_gene);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_fitness_fn(individual: &Individual) -> i32 {
        // TODO
        individual.get_fitness().clone()
    }

    #[test]
    fn test_model() {
        let model = Model::new(
            Population::new(vec![
                Individual::new(vec![1, 2, 3]),
                Individual::new(vec![1, 2, 3]),
            ]),
            &test_fitness_fn,
        );

        // TOOD: model validity
        let exp_pop_genes = vec![1, 2, 3, 1, 2, 3];
        let res_pop_genes: Vec<u16> = model
            .population
            .get_individuals()
            .into_iter()
            .flat_map(|i| i.get_genes().clone().into_iter())
            .collect();

        assert_eq!(res_pop_genes, exp_pop_genes);
        // TODO: update after fitness fn is implemented
        assert_eq!(
            (model.fitness_fn)(&Individual::new(vec![1, 2, 3])),
            i32::MIN
        );
    }

    #[test]
    fn test_score_population() {
        let mut model = Model::new(
            Population::new(vec![
                Individual::new(vec![1, 2, 3]),
                Individual::new(vec![1, 2, 3]),
            ]),
            &test_fitness_fn,
        );

        // TODO: update after a fitness_fn is implemented
        model.score_population();

        for individual in model.population.get_individuals() {
            assert_eq!(individual.get_fitness().to_owned(), i32::MIN);
        }
    }

    #[test]
    fn test_selection() {
        let mut i1 = Individual::new(vec![1, 2, 3]);
        i1.update_fitness_score(-1);

        let mut i2 = Individual::new(vec![4, 5, 6]);
        i2.update_fitness_score(1);

        let mut model = Model::new(Population::new(vec![i1, i2]), &test_fitness_fn);

        let selection_rate = 0.5;
        let results = model.select_for_reproduction(selection_rate);

        assert_eq!(results[0].get_genes().to_owned(), vec![4, 5, 6]);
    }

    #[test]
    fn test_crossover() {
        let parent_a = Individual::new(vec![0, 0, 0]);
        let parent_b = Individual::new(vec![1, 1, 1]);
        let model = Model::new(
            Population::new(vec![
                Individual::new(parent_a.get_genes().clone()),
                Individual::new(parent_b.get_genes().clone()),
            ]),
            &test_fitness_fn,
        );

        let crossover_rate = 0.5;
        let res = model.reproduce(parent_a, parent_b, crossover_rate);

        assert_eq!(res.get_genes().to_owned(), vec![0, 1, 1]);
    }

    #[test]
    fn test_mutation() {
        // TODO
        assert!(true);
    }
}
