/// These ga solvers are designed to solve both constrained and unconstrained problems by encoding solution traits as
/// *genes*, solution states as *individuals*, and solution groups as *populations*. Genetic algorithms improve on populations
/// iteratively (referred to as *generations*) via reproduction and scoring an individual's *fitness*.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gene() {
        let gene = Gene::new("Test", vec![1, 2, 3]);
        let expected = vec![1, 2, 3];

        assert_eq!(&gene.data, &expected);
        assert_eq!(gene.name, "Test");
    }

    #[test]
    fn individual() {
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
    fn population() {
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
    fn model() {
        fn fitness(individual: Individual) -> i32 {
            0 // TODO
        }

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
            &fitness,
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
}
