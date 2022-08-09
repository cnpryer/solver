/// An `Individual` is a structure that represents a solution's composition.
///
/// For example, in scheduling problems an individual could represent a schedule.
#[derive(Clone)]
pub struct Individual {
    /// The genes of an individual represented as positive integers.
    genes: Vec<u32>,
    /// The fitness (score) of an individual.
    fitness: u32,
}

impl Individual {
    pub fn new(genes: Vec<u32>, fitness: u32) -> Individual {
        Individual { genes, fitness }
    }

    pub fn update_fitness_score(&mut self, score: u32) {
        self.fitness = score;
    }

    pub fn get_genes(&self) -> &Vec<u32> {
        &self.genes
    }

    pub fn get_fitness(&self) -> &u32 {
        &self.fitness
    }

    /// Update a gene at a specific position.
    pub fn update_gene(&mut self, pos: usize, gene: u32) {
        self.genes[pos] = gene;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_individual() {
        let individual = Individual::new(vec![1, 2, 3], u32::MIN);

        // TODO: implement equality
        let expected = vec![1, 2, 3];
        let res: Vec<u32> = individual.genes.into_iter().collect();

        assert_eq!(res, expected);
        assert_eq!(individual.fitness, u32::MIN);
    }
}
