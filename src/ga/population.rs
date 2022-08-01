use crate::ga::individual::Individual;

/// A `Population` is a group of `Individual`s.
#[derive(Clone)]
pub struct Population {
    generation: u32,
    individuals: Vec<Individual>,
}

impl Population {
    pub fn new(generation: u32, individuals: Vec<Individual>) -> Population {
        Population {
            generation,
            individuals,
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

    pub fn update_individuals(&mut self, individuals: Vec<Individual>) {
        self.individuals = individuals
    }

    /// Sort individuals by their fitness score in descending order.
    pub(crate) fn sort_by_fitness(&mut self) {
        self.individuals
            .sort_by(|a, b| b.get_fitness().cmp(a.get_fitness()));
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
        let res: Vec<u16> = population
            .individuals
            .into_iter()
            .flat_map(|i| i.get_genes().clone().into_iter())
            .collect();

        assert_eq!(res, expected);
        assert_eq!(population.generation, 0);
    }
}
