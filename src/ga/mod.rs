use std::error::Error;

/// These ga solvers are designed to solve both constrained and unconstrained problems by encoding solution traits as
/// *genes*, solution states as *individuals*, and solution groups as *populations*. Genetic algorithms improve on populations
/// iteratively (referred to as *generations*) via reproduction and scoring an individual's *fitness*.

const SOLVER_NAME: &str = "GeneticAlgorithm";

// Default data for genetic algorithm configuration
const DEFAULT_MINIMUM_GENERATIONS: u32 = 1000;
const DEFAULT_SATISFACTORY_FITNESS: u32 = 0;
const DEFAULT_CROSSOVER_RATE: f32 = 0.5;
const DEFAULT_MUTATION_RATE: f32 = 0.05;
const DEFAULT_SELECTION_RATE: f32 = 0.5;

/// A `Gene` is the atomic structure of the evolutionary solver used to represent solution traits.
///
/// For example, in scheduling problems a gene could represent time, duration, etc..
pub struct Gene<'a> {
    name: &'a str,
    data: Vec<u16>,
}

impl Gene<'_> {
    fn new(name: &str, data: Vec<u16>) -> Gene {
        Gene { name, data }
    }
}

/// An `Individual` is a structure that represents a solution's composition. `Individual`s are composed of `Gene`s.
///
/// For example, in scheduling problems an individual could represent a schedule.
pub struct Individual<'a> {
    // Ordered (TODO) vector of genes. Each individual in a population must have the same types of genes
    genes: Vec<Gene<'a>>,
    fitness: Option<i32>,
}

impl Individual<'_> {
    fn new(genes: Vec<Gene>) -> Individual {
        Individual {
            genes,
            fitness: None,
        }
    }
}

/// A `Population` is a group of `Individual`s.
pub struct Population<'a> {
    generation: u32,
    individuals: Vec<Individual<'a>>,
}

impl Population<'_> {
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
    population: Population<'a>,
    // fitness evaluation function that evaluates an Individual
    fitness: &'a dyn Fn(Individual) -> i32,
}

impl Model<'_> {
    fn new<'a>(population: Population<'a>, fitness: &'a dyn Fn(Individual) -> i32) -> Model<'a> {
        Model {
            population,
            fitness,
        }
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

    fn test_fitness_fn(individual: Individual) -> i32 {
        0 // TODO
    }

    #[test]
    fn test_gene() {
        let gene = Gene::new("Test", vec![1, 2, 3]);
        let expected = vec![1, 2, 3];

        assert_eq!(&gene.data, &expected);
        assert_eq!(gene.name, "Test");
    }

    #[test]
    fn test_individual() {
        let individual = Individual::new(vec![
            Gene::new("Gene0", vec![1, 2, 3]),
            Gene::new("Gene1", vec![1, 2, 3]),
        ]);

        // TODO: implement equality
        let expected = vec![1, 2, 3, 1, 2, 3];
        let res: Vec<u16> = individual
            .genes
            .into_iter()
            .flat_map(|g| g.data.into_iter())
            .collect();

        assert_eq!(res, expected);
        assert_eq!(individual.fitness, None);
    }

    #[test]
    fn test_population() {
        let population = Population::new(vec![
            Individual::new(vec![
                Gene::new("Gene0", vec![1, 2, 3]),
                Gene::new("Gene1", vec![1, 2, 3]),
            ]),
            Individual::new(vec![
                Gene::new("Gene0", vec![1, 2, 3]),
                Gene::new("Gene1", vec![1, 2, 3]),
            ]),
        ]);

        // TODO: implement equality
        let expected = vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3];
        let res: Vec<u16> = population
            .individuals
            .into_iter()
            .flat_map(|i| i.genes.into_iter().flat_map(|g| g.data.into_iter()))
            .collect();

        assert_eq!(res, expected);
        assert_eq!(population.generation, 0);
    }

    #[test]
    fn test_model() {
        let model = Model::new(
            Population::new(vec![
                Individual::new(vec![
                    Gene::new("Gene0", vec![1, 2, 3]),
                    Gene::new("Gene1", vec![1, 2, 3]),
                ]),
                Individual::new(vec![
                    Gene::new("Gene0", vec![1, 2, 3]),
                    Gene::new("Gene1", vec![1, 2, 3]),
                ]),
            ]),
            &test_fitness_fn,
        );

        // TOOD: model validity
        let exp_pop_genes = vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3];
        let res_pop_genes: Vec<u16> = model
            .population
            .individuals
            .into_iter()
            .flat_map(|i| i.genes.into_iter().flat_map(|g| g.data.into_iter()))
            .collect();

        assert_eq!(res_pop_genes, exp_pop_genes);
        // TODO: update after fitness fn is implemented
        assert_eq!(
            (model.fitness)(Individual::new(vec![Gene::new("Gene0", vec![1, 2, 3])])),
            0
        );
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
                Individual::new(vec![
                    Gene::new("Gene0", vec![1, 2, 3]),
                    Gene::new("Gene1", vec![1, 2, 3]),
                ]),
                Individual::new(vec![
                    Gene::new("Gene0", vec![1, 2, 3]),
                    Gene::new("Gene1", vec![1, 2, 3]),
                ]),
            ]),
            &test_fitness_fn,
        );
        let test_solver = Solver::new(model, None);

        // TODO
        run(&test_solver);

        assert_eq!(test_solver.name, SOLVER_NAME);
    }
}
