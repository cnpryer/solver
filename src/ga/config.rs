// Default data for genetic algorithm configuration
const DEFAULT_MINIMUM_GENERATIONS: u32 = 1000;
const DEFAULT_SATISFACTORY_FITNESS: u32 = 0;
const DEFAULT_CROSSOVER_RATE: f32 = 0.5;
const DEFAULT_MUTATION_RATE: f32 = 0.05;
const DEFAULT_SELECTION_RATE: f32 = 0.5;

// A `Config` used to configure the solve.
pub struct Config {
    // Minumum number of generations to produce
    generations: u32,
    // Minimum threshold for solve defined as fitness score
    fitness_threshold: Option<u32>,
    // Percent of parent A to crossover with parent B
    crossover_rate: f32,
    // Rate at which genes are psuedo-logically mutated
    mutation_rate: f32,
    // Rate at which indivudals are selected for reproduction
    selection_rate: f32,
}

impl Config {
    pub fn new(
        generations: u32,
        fitness_threshold: Option<u32>,
        crossover_rate: f32,
        mutation_rate: f32,
        selection_rate: f32,
    ) -> Self {
        Config {
            generations,
            fitness_threshold,
            crossover_rate,
            mutation_rate,
            selection_rate,
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            generations: DEFAULT_MINIMUM_GENERATIONS,
            fitness_threshold: Some(DEFAULT_SATISFACTORY_FITNESS),
            crossover_rate: DEFAULT_CROSSOVER_RATE,
            mutation_rate: DEFAULT_MUTATION_RATE,
            selection_rate: DEFAULT_SELECTION_RATE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let cfg = Config::default();

        assert_eq!(cfg.crossover_rate, DEFAULT_CROSSOVER_RATE);
        assert_eq!(cfg.fitness_threshold, Some(DEFAULT_SATISFACTORY_FITNESS));
        assert_eq!(cfg.generations, DEFAULT_MINIMUM_GENERATIONS);
        assert_eq!(cfg.mutation_rate, DEFAULT_MUTATION_RATE);
        assert_eq!(cfg.selection_rate, DEFAULT_SELECTION_RATE);
    }
}
