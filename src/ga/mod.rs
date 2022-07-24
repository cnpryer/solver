/// These ga solvers are designed to solve both constrained and unconstrained problems by encoding solution traits as
/// *genes*, solution states as *individuals*, and solution groups as *populations*. Genetic algorithms improve on populations
/// iteratively (referred to as *generations*) via reproduction and scoring an individual's *fitness*.
///
/// `Gene`
/// A `Gene` is the atomic structure of the evolutionary solver used to represent solution traits.
///
/// For example, in scheduling problems a gene could represent time, duration, etc..
///
/// `Individual`
/// An `Individual` is a structure that represents a solution's composition. `Individual`s are composed of `Gene`s.
///
/// For example, in scheduling problems an individual could represent a schedule.
///
/// `Population`
/// A `Population` is a group of `Individual`s.
///
/// `Model`
/// A `Model` is a structure that defines the problem to be solved.
pub struct Gene {
    data: Vec<u16>,
}

impl Gene {
    fn new(data: Vec<u16>) -> Gene {
        Gene { data }
    }
}

pub struct Individual {
    genes: Vec<Gene>,
}

impl Individual {
    fn new(genes: Vec<Gene>) -> Individual {
        Individual { genes }
    }
}

pub struct Population {
    individuals: Vec<Individual>,
}

impl Population {
    fn new(individuals: Vec<Individual>) -> Population {
        Population { individuals }
    }
}

pub struct Model<'a> {
    // first generation of individuals
    population: Population,
    // fitness evaluation function that evaluates an Individual
    fitness: &'a dyn Fn(Individual) -> i32,
}

impl Model<'_> {
    fn new(population: Population, fitness: &dyn Fn(Individual) -> i32) -> Model {
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
        let gene = Gene::new(vec![1, 2, 3]);
        let expected = vec![1, 2, 3];

        assert_eq!(&gene.data, &expected);
    }

    #[test]
    fn individual() {
        let individual = Individual::new(vec![Gene::new(vec![1, 2, 3]), Gene::new(vec![1, 2, 3])]);

        // TODO: implement equality
        let expected = vec![1, 2, 3, 1, 2, 3];
        let res: Vec<u16> = individual
            .genes
            .into_iter()
            .flat_map(|g| g.data.into_iter())
            .collect();

        assert_eq!(res, expected);
    }

    #[test]
    fn population() {
        let population = Population::new(vec![
            Individual::new(vec![Gene::new(vec![1, 2, 3]), Gene::new(vec![1, 2, 3])]),
            Individual::new(vec![Gene::new(vec![1, 2, 3]), Gene::new(vec![1, 2, 3])]),
        ]);

        // TODO: implement equality
        let expected = vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3];
        let res: Vec<u16> = population
            .individuals
            .into_iter()
            .flat_map(|i| i.genes.into_iter().flat_map(|g| g.data.into_iter()))
            .collect();

        assert_eq!(res, expected);
    }

    #[test]
    fn model() {
        fn fitness(individual: Individual) -> i32 {
            0 // TODO
        }

        let model = Model::new(
            Population::new(vec![
                Individual::new(vec![Gene::new(vec![1, 2, 3]), Gene::new(vec![1, 2, 3])]),
                Individual::new(vec![Gene::new(vec![1, 2, 3]), Gene::new(vec![1, 2, 3])]),
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
    }
}
