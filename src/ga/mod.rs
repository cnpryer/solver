/// `ga` is designed to solve both constrained and unconstrained problems by encoding solution traits as
/// *genes*, solution states as *individuals*, and solution groups as *populations*. Genetic algorithms improve on populations
/// iteratively (referred to as *generations*) via reproduction and scoring an individual's *fitness*.
use rand::{thread_rng, Rng};

const SOLVER_NAME: &str = "GeneticAlgorithm";

// Default data for genetic algorithm configuration
const DEFAULT_MINIMUM_GENERATIONS: u32 = 1000;
const DEFAULT_SATISFACTORY_FITNESS: u32 = 0;
const DEFAULT_CROSSOVER_RATE: f32 = 0.5;
const DEFAULT_MUTATION_RATE: f32 = 0.05;
const DEFAULT_SELECTION_RATE: f32 = 0.5;

/// An `Individual` is a structure that represents a solution's composition.
///
/// For example, in scheduling problems an individual could represent a schedule.
#[derive(Clone)]
pub struct Individual {
    genes: Vec<u16>,
    fitness: Option<i32>,
}

impl Individual {
    fn new(genes: Vec<u16>) -> Individual {
        Individual {
            genes,
            fitness: None,
        }
    }

    fn update_fitness_score(&mut self, score: i32) {
        self.fitness = Some(score);
    }
}

/// A `Population` is a group of `Individual`s.
pub struct Population {
    generation: u32,
    individuals: Vec<Individual>,
}

impl Population {
    fn new(individuals: Vec<Individual>) -> Population {
        Population {
            generation: 0,
            individuals,
        }
    }
}

/// A `Model` is a structure that defines the problem to be solved.
pub struct Model<'a> {
    // first generation of individuals
    population: Population,
    // fitness evaluation function that evaluates an Individual
    fitness_fn: &'a dyn Fn(&Individual) -> i32,
}

impl Model<'_> {
    fn new<'a>(population: Population, fitness_fn: &'a dyn Fn(&Individual) -> i32) -> Model<'a> {
        Model {
            population,
            fitness_fn,
        }
    }

    /// Applies a new fitness score to each individual in the population.
    fn score_population(&mut self) {
        for i in 0..self.population.individuals.len() {
            let score = (self.fitness_fn)(&self.population.individuals[i]);
            self.population.individuals[i].update_fitness_score(score);
        }
    }

    fn sort_population_by_fitness(&mut self) {
        self.population
            .individuals
            .sort_by(|a, b| b.fitness.cmp(&a.fitness));
    }

    /// Selects a subset of the modeled population based on fitness scores and a `selection_rate`.
    fn select_for_reproduction(&mut self, selection_rate: f32) -> Vec<Individual> {
        // Sort the population individuals in descending order based on their fitness scores
        self.sort_population_by_fitness(); // TODO: Avoid this by retaining pre-sorted pop

        let n = ((self.population.individuals.len() as f32) * selection_rate) as usize;
        self.population.individuals[..n].to_vec()
    }

    /// Breed two parents using a `crossover_rate`.
    fn reproduce(
        &self,
        parent_a: Individual,
        parent_b: Individual,
        crossover_rate: f32,
    ) -> Individual {
        // `crossover_rate` is used to slice n-length of `parent_a` and the remaining length of `parent_b`
        let n = ((parent_a.genes.len() as f32) * crossover_rate) as usize;
        let mut new_genes = parent_a.genes[0..n].to_vec();
        new_genes.extend(&parent_b.genes[n..]);

        Individual {
            genes: new_genes,
            fitness: None,
        }
    }

    /// Mutates an individual using a `mutation_rate`.
    fn mutate_individual(&mut self, individual: &mut Individual, gene_pool: Vec<u16>) {
        let mut rng = thread_rng();
        let new_gene = gene_pool[rng.gen_range(0..gene_pool.len())];
        let n = individual.genes.len();
        individual.genes[rng.gen_range(0..n)] = new_gene;
    }
}

// A `Config` used to configure the solve.
struct Config {
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
    fn new(
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

pub struct Solver<'a> {
    name: &'a str,
    model: Model<'a>,
    config: Option<Config>,
}

impl Solver<'_> {
    fn new(model: Model<'_>, config: Option<Config>) -> Solver {
        Solver {
            name: SOLVER_NAME,
            model,
            config,
        }
    }
}

// TODO
fn run(solver: &Solver) -> () {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_fitness_fn(individual: &Individual) -> i32 {
        // TODO
        if individual.fitness.is_none() {
            return 0;
        }

        individual.fitness.unwrap()
    }

    #[test]
    fn test_individual() {
        let individual = Individual::new(vec![1, 2, 3]);

        // TODO: implement equality
        let expected = vec![1, 2, 3];
        let res: Vec<u16> = individual.genes.into_iter().collect();

        assert_eq!(res, expected);
        assert_eq!(individual.fitness, None);
    }

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
            .flat_map(|i| i.genes.into_iter())
            .collect();

        assert_eq!(res, expected);
        assert_eq!(population.generation, 0);
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
            .individuals
            .into_iter()
            .flat_map(|i| i.genes.into_iter())
            .collect();

        assert_eq!(res_pop_genes, exp_pop_genes);
        // TODO: update after fitness fn is implemented
        assert_eq!((model.fitness_fn)(&Individual::new(vec![1, 2, 3])), 0);
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

        for individual in model.population.individuals {
            assert_eq!(individual.fitness, Some(0));
        }
    }

    #[test]
    fn test_selection() {
        let mut moved_individual = Individual::new(vec![1, 2, 3]);
        moved_individual.update_fitness_score(-1);
        let mut model = Model::new(
            Population::new(vec![moved_individual, Individual::new(vec![4, 5, 6])]),
            &test_fitness_fn,
        );
        model.score_population();

        let selection_rate = 0.5;
        let results = model.select_for_reproduction(selection_rate);

        assert_eq!(results[0].genes, vec![4, 5, 6]);
    }

    #[test]
    fn test_crossover() {
        let parent_a = Individual::new(vec![0, 0, 0]);
        let parent_b = Individual::new(vec![1, 1, 1]);
        let model = Model::new(
            Population::new(vec![
                Individual::new(parent_a.genes.clone()),
                Individual::new(parent_b.genes.clone()),
            ]),
            &test_fitness_fn,
        );

        let crossover_rate = 0.5;
        let res = model.reproduce(parent_a, parent_b, crossover_rate);

        assert_eq!(res.genes, vec![0, 1, 1]);
    }

    #[test]
    fn test_mutation() {
        // TODO
        assert!(true);
    }

    #[test]
    fn test_config() {
        let cfg = Config::default();

        assert_eq!(cfg.crossover_rate, DEFAULT_CROSSOVER_RATE);
        assert_eq!(cfg.fitness_threshold, Some(DEFAULT_SATISFACTORY_FITNESS));
        assert_eq!(cfg.generations, DEFAULT_MINIMUM_GENERATIONS);
        assert_eq!(cfg.mutation_rate, DEFAULT_MUTATION_RATE);
        assert_eq!(cfg.selection_rate, DEFAULT_SELECTION_RATE);
    }

    #[test]
    fn test_solver() {
        let model = Model::new(
            Population::new(vec![
                Individual::new(vec![1, 2, 3]),
                Individual::new(vec![1, 2, 3]),
            ]),
            &test_fitness_fn,
        );
        let test_solver = Solver::new(model, None);

        // TODO
        run(&test_solver);

        assert_eq!(test_solver.name, SOLVER_NAME);
    }
}
