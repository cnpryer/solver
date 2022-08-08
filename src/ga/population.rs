use std::collections::HashSet;

use crate::ga::individual::Individual;

/// A `Population` is a group of `Individual`s.
#[derive(Clone)]
pub struct Population {
    /// Generation number of the population.
    generation: u32,
    /// The individuals of the population.
    individuals: Vec<Individual>,
    /// Normalized fitness scores for every individual in the population (contains values 0 to 1).
    normalized_scores: Option<Vec<f32>>,
}

impl Population {
    pub fn new(generation: u32, individuals: Vec<Individual>) -> Population {
        Population {
            generation,
            individuals,
            normalized_scores: None,
        }
    }

    pub fn set_generation(&mut self, generation: u32) {
        self.generation = generation;
    }

    pub fn get_individuals(&self) -> &Vec<Individual> {
        &self.individuals
    }

    pub fn get_generation(&self) -> &u32 {
        &self.generation
    }

    pub fn get_normalized_scores(&self) -> &Option<Vec<f32>> {
        &self.normalized_scores
    }

    /// Normalize fitness scores to values between 0 and 1.
    pub(crate) fn normalize_fitness_scores(&mut self) {
        let mut scores: Vec<f32> = self
            .individuals
            .iter()
            .map(|i| *i.get_fitness() as f32)
            .collect();
        let total: f32 = scores.iter().map(|s| *s as f32).sum();
        scores = scores.into_iter().map(|s| s / total).collect();

        self.normalized_scores = Some(scores)
    }

    pub fn update_individuals(&mut self, individuals: Vec<Individual>) {
        self.individuals = individuals
    }

    /// Sort individuals by their fitness score in descending order.
    pub(crate) fn sort_by_fitness(&mut self) {
        self.individuals
            .sort_by(|a, b| b.get_fitness().cmp(a.get_fitness()));
    }

    /// Get unique genes from the population.
    pub(crate) fn get_gene_pool(&self) -> HashSet<u32> {
        let mut genes = HashSet::new();

        for indiviudal in self.get_individuals() {
            for gene in indiviudal.get_genes() {
                let _ = genes.insert(*gene);
            }
        }

        genes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_population() {
        let population = Population::new(
            0,
            vec![
                Individual::new(vec![1, 2, 3], i32::MIN),
                Individual::new(vec![1, 2, 3], i32::MIN),
            ],
        );

        // TODO: implement equality
        let expected = vec![1, 2, 3, 1, 2, 3];
        let res: Vec<u32> = population
            .individuals
            .into_iter()
            .flat_map(|i| i.get_genes().clone().into_iter())
            .collect();

        assert_eq!(res, expected);
        assert_eq!(population.generation, 0);
    }

    #[test]
    fn test_normalize_fitness_scores() {
        let mut population = Population::new(
            0,
            vec![
                Individual::new(vec![1, 2, 3], 2),
                Individual::new(vec![1, 2, 3], 2),
            ],
        );

        population.normalize_fitness_scores();

        let expected = Some(vec![0.5, 0.5]);
        assert_eq!(population.normalized_scores, expected);
    }
}
