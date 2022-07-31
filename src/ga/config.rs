// Default data for genetic algorithm configuration
const DEFAULT_MINIMUM_GENERATIONS: u32 = 1000;
const DEFAULT_SATISFACTORY_FITNESS: u32 = 0;
const DEFAULT_CROSSOVER_RATE: f32 = 0.5;
const DEFAULT_MUTATION_RATE: f32 = 0.05;
const DEFAULT_SELECTION_RATE: f32 = 0.5;

// A `Config` used to configure the solve.
pub struct Config {
    // Minumum number of generations to produce
    pub generations: u32,
    // Minimum threshold for solve defined as fitness score
    pub fitness_threshold: Option<u32>,
    // Percent of parent A to crossover with parent B
    pub crossover_rate: f32,
    // Rate at which genes are psuedo-logically mutated
    pub mutation_rate: f32,
    // Rate at which indivudals are selected for reproduction
    pub selection_rate: f32,
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
    fn test_default_config() {
        let config = Config::default();

        assert_eq!(config.crossover_rate, DEFAULT_CROSSOVER_RATE);
        assert_eq!(config.fitness_threshold, Some(DEFAULT_SATISFACTORY_FITNESS));
        assert_eq!(config.generations, DEFAULT_MINIMUM_GENERATIONS);
        assert_eq!(config.mutation_rate, DEFAULT_MUTATION_RATE);
        assert_eq!(config.selection_rate, DEFAULT_SELECTION_RATE);
    }

    #[test]
    fn test_custom_config() {
        let generations = 1000;
        let fitness_threshold = Some(1000);
        let crossover_rate = 0.5;
        let mutation_rate = 0.5;
        let selection_rate = 0.5;

        let config = Config::new(
            generations,
            fitness_threshold,
            crossover_rate,
            mutation_rate,
            selection_rate,
        );

        assert_eq!(config.generations, generations);
        assert_eq!(config.fitness_threshold, fitness_threshold);
        assert_eq!(config.crossover_rate, crossover_rate);
        assert_eq!(config.mutation_rate, mutation_rate);
        assert_eq!(config.selection_rate, selection_rate);
    }
}
