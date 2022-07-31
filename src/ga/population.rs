use crate::ga::individual::Individual;

/// A `Population` is a group of `Individual`s.
pub struct Population {
    generation: u32,
    individuals: Vec<Individual>,
}

impl Population {
    pub fn new(individuals: Vec<Individual>) -> Population {
        Population {
            generation: 0,
            individuals,
        }
    }

    pub fn get_individuals(&self) -> &Vec<Individual> {
        &self.individuals
    }

    pub fn update_individuals(&mut self, individuals: Vec<Individual>) {
        self.individuals = individuals
    }

    pub fn sort_by_fitness(&mut self) {
        self.individuals
            .sort_by(|a, b| b.get_fitness().cmp(a.get_fitness()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_population() {
        let population = Population::new(vec![
            Individual::new(vec![1, 2, 3]),
            Individual::new(vec![1, 2, 3]),
        ]);

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