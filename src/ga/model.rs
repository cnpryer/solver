use rand::{thread_rng, Rng};

use crate::ga::config::Config;
use crate::ga::individual::Individual;
use crate::ga::population::Population;

/// A `Model` is a structure that defines the problem to be solved.
pub struct Model<'a> {
    // A group of individuals
    population: Population,
    // Function used to evaluate an individual's *fitness*
    fitness_fn: &'a dyn Fn(&Individual) -> i32,
    // Configuration for the model
    config: Config,
}

impl Model<'_> {
    pub fn new(
        population: Population,
        fitness_fn: &'_ dyn Fn(&Individual) -> i32,
        config: Config,
    ) -> Model {
        Model {
            population,
            fitness_fn,
            config,
        }
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_population(&self) -> &Population {
        &self.population
    }

    /// Apply a new fitness score to each individual in the population.
    fn score_population(&mut self) {
        // TODO: It's probably unnecessarily expensive to clone each individual like this
        //       just to update their fitness scores.
        let mut individuals = self.population.get_individuals().clone();

        for individual in &mut individuals {
            let score = (self.fitness_fn)(individual);
            individual.update_fitness_score(score);
        }

        self.population.update_individuals(individuals);
    }

    /// Normalize fitness scores to values between 0 and 1.
    fn _normalize_scores(&mut self) {
        unimplemented!()
    }

    /// Selects a subset of the modeled population based on fitness scores and the configured selection rate.
    /// This function assumes the population is pre-sorted by fitness scores
    /// TODO: Use probability based on fitness scores rather than a sorted truncation-like selection.
    fn select_for_reproduction(&mut self) -> Vec<Individual> {
        // Get top-n individuals using `selection_rate`
        let n = ((self.population.get_individuals().len() as f32) * self.config.selection_rate)
            as usize;
        self.population.get_individuals()[..n].to_vec()
    }

    /// Create a new `Individual` by breeding two parents using the configured crossover rate.
    fn reproduce(&self, parent_a: &Individual, parent_b: &Individual) -> Individual {
        // `crossover_rate` is used to slice n-length of `parent_a` and the remaining length of `parent_b`
        let n = ((parent_a.get_genes().len() as f32) * self.config.crossover_rate) as usize;
        let mut new_genes = parent_a.get_genes()[0..n].to_vec();
        new_genes.extend(&parent_b.get_genes()[n..]);

        Individual::new(new_genes, i32::MIN)
    }

    /// Randomly modifies an `Individual` from a pool of genes.
    /// TODO: Use
    fn _mutate_individual(&mut self, individual: &mut Individual, gene_pool: Vec<u16>) {
        // Pull random gene from the `gene_pool`
        let mut rng = thread_rng();
        let i = rng.gen_range(0..gene_pool.len());
        let new_gene = gene_pool[i];

        // Update a random gene from the individual
        let n = individual.get_genes().len();
        let i = rng.gen_range(0..n);
        individual.update_gene(i, new_gene);
    }

    /// TODO: Need to update the implementation for corrections (See notes).
    /// Run the model.
    /// The first population is assumed to be initialized randomly as the 0th generation. The configured
    /// selection rate determines the subset of the population (fitess-dependent) that is selected to
    /// reproduce. Fitness scores are normalized with each generation. Each individual's fitness score
    /// is used to determine the probability of their selection for reproduction. During reproduction
    /// the configured crossover rate is used to determine how much of Parent A's genes are passed to
    /// the offspring and the remaining genes are carried over from Parent B. With each new generation
    /// the configured mutation rate determines the subset of the new population which each individual's
    /// genes are then randomly mutated. The model stops generating new populations based on the configured
    /// exit parameters.
    pub fn run(&mut self) {
        let initial_generation_num = self.population.get_generation();
        let population_size = self.population.get_individuals().len();

        // Build populations from initial generation
        for generation in (*initial_generation_num + 1)..self.config.max_generations {
            // Sort the initial population by their fitness scores
            // TODO: Don't do this when using probability
            self.score_population();
            self.population.sort_by_fitness();

            let parents = self.select_for_reproduction();

            // One child is produced for each pair of parents
            let mut offspring = Vec::with_capacity(parents.len() / 2);

            for i in (0..parents.len()).step_by(2) {
                let j = i + 1;

                if j > parents.len() - 1 {
                    break;
                }

                let parent_a = &parents[i];
                let parent_b = &parents[j];

                let child = self.reproduce(parent_a, parent_b);
                offspring.push(child);
            }

            // Each population must be the same size as the initial generation
            // Therefore a survival subset needs to be retained in addition to the offspring
            // TODO: Use probability
            let survivors =
                self.population.get_individuals()[0..(population_size - offspring.len())].to_vec();

            offspring.extend(survivors);

            // TODO: Mutation

            self.population = Population::new(generation, offspring);
        }

        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_fitness_fn(individual: &Individual) -> i32 {
        // TODO
        individual.get_fitness().clone()
    }

    #[test]
    fn test_model() {
        let model = Model::new(
            Population::new(
                0,
                vec![
                    Individual::new(vec![1, 2, 3], i32::MIN),
                    Individual::new(vec![1, 2, 3], i32::MIN),
                ],
            ),
            &mock_fitness_fn,
            Config::default(),
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
            (model.fitness_fn)(&Individual::new(vec![1, 2, 3], i32::MIN)),
            i32::MIN
        );
    }

    #[test]
    fn test_score_population() {
        let mut model = Model::new(
            Population::new(
                0,
                vec![
                    Individual::new(vec![1, 2, 3], i32::MIN),
                    Individual::new(vec![1, 2, 3], i32::MIN),
                ],
            ),
            &mock_fitness_fn,
            Config::default(),
        );

        // TODO: update after a fitness_fn is implemented
        model.score_population();

        for individual in model.population.get_individuals() {
            assert_eq!(individual.get_fitness().to_owned(), i32::MIN);
        }
    }

    #[test]
    fn test_selection() {
        let i1 = Individual::new(vec![1, 2, 3], -1);
        let i2 = Individual::new(vec![4, 5, 6], 1);
        let mut model = Model::new(
            Population::new(0, vec![i1, i2]),
            &mock_fitness_fn,
            Config::default(),
        );

        // NOTE: Uses default selection rate
        model.score_population();
        model.population.sort_by_fitness();
        let results = model.select_for_reproduction();

        assert_eq!(results[0].get_genes().to_owned(), vec![4, 5, 6]);
    }

    #[test]
    fn test_crossover() {
        let parent_a = Individual::new(vec![0, 0, 0], i32::MIN);
        let parent_b = Individual::new(vec![1, 1, 1], i32::MIN);
        let model = Model::new(
            Population::new(
                0,
                vec![
                    Individual::new(parent_a.get_genes().clone(), i32::MIN),
                    Individual::new(parent_b.get_genes().clone(), i32::MIN),
                ],
            ),
            &mock_fitness_fn,
            Config::default(),
        );

        // NOTE: Uses default selection rate
        let res = model.reproduce(&parent_a, &parent_b);

        assert_eq!(res.get_genes().to_owned(), vec![0, 1, 1]);
    }

    #[test]
    fn test_run() {
        assert!(true);
        // TODO
        // let mut model = Model::new(
        //     Population::new(
        //         0,
        //         vec![
        //             Individual::new(vec![1, 2, 3, 4], i32::MIN),
        //             Individual::new(vec![5, 6, 7, 8], i32::MIN),
        //         ],
        //     ),
        //     &mock_fitness_fn,
        //     Config::default(),
        // );

        // model.run();
    }

    #[test]
    fn test_mutation() {
        // TODO
        assert!(true);
    }
}
