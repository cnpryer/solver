/// An `Individual` is a structure that represents a solution's composition.
///
/// For example, in scheduling problems an individual could represent a schedule.
#[derive(Clone)]
pub struct Individual {
    genes: Vec<u16>,
    fitness: i32,
}

impl Individual {
    pub fn new(genes: Vec<u16>) -> Individual {
        Individual {
            genes,
            fitness: i32::MIN,
        }
    }

    pub fn update_fitness_score(&mut self, score: i32) {
        self.fitness = score;
    }

    pub fn get_genes(&self) -> &Vec<u16> {
        &self.genes
    }

    pub fn get_fitness(&self) -> &i32 {
        &self.fitness
    }

    pub fn update_gene(&mut self, pos: usize, gene: u16) {
        self.genes[pos] = gene;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_fitness_fn(individual: &Individual) -> i32 {
        // TODO
        individual.fitness
    }

    #[test]
    fn test_individual() {
        let individual = Individual::new(vec![1, 2, 3]);

        // TODO: implement equality
        let expected = vec![1, 2, 3];
        let res: Vec<u16> = individual.genes.into_iter().collect();

        assert_eq!(res, expected);
        assert_eq!(individual.fitness, i32::MIN);
    }
}
