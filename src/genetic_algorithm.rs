use rand::{thread_rng, Rng};

const GENES: [u8; 86] =
    *b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ 1234567890, .-;:_!\"#%&/()=?@${[]}";

/// Produce a muated gene
pub fn mutated_genes() -> u8 {
    let rand_index = thread_rng().gen_range(0..(GENES.len() as u32)) as usize;
    GENES[rand_index]
}

/// Struct that represent one individual of the population
#[derive(Clone)]
pub struct Individual {
    genome: Vec<u8>,
    target: Vec<u8>,
    fitness: u32,
}

pub trait IndividualTrait {
    fn new(target: &Vec<u8>) -> Individual;
    fn mate(&self, parent2: &Individual) -> Individual;
    fn cal_fitness(&mut self);
    fn get_genome(&self) -> Vec<u8>;
    fn get_fitness(&self) -> u32;
}

impl IndividualTrait for Individual {
    /// Function for producing one Individual
    fn new(target: &Vec<u8>) -> Individual {
        let mut individual = Individual {
            genome: vec![],
            target: target.clone(),
            fitness: 0,
        };

        (0..target.len()).for_each(|_| individual.genome.push(mutated_genes()));
        individual.cal_fitness();
        individual
    }

    /// Perform mating for producing a new offwpring
    fn mate(&self, parent2: &Individual) -> Individual {
        let mut individual = Individual {
            genome: self.genome.clone(),
            target: self.target.clone(),
            fitness: 0,
        };

        (0..self.genome.len()).for_each(|i| {
            // Preserve the good genes
            // More efficient
            if self.genome[i] != self.target[i] {
                let prob = thread_rng().gen::<f32>();
                if 0.45f32 <= prob && prob < 0.9f32 {
                    individual.genome[i] = parent2.genome[i];
                } else {
                    individual.genome[i] = mutated_genes();
                }
            }
        });
        individual.cal_fitness();
        individual
    }

    /// Calculate the number letters that doesn't match to the target letters
    fn cal_fitness(&mut self) {
        let mut fitness: u32 = 0;
        (0..self.genome.len()).for_each(|i| {
            if self.genome[i] != self.target[i] {
                fitness += 1;
            }
        });
        self.fitness = fitness;
    }

    /// Get the fitness score
    fn get_fitness(&self) -> u32 {
        self.fitness
    }

    fn get_genome(&self) -> Vec<u8> {
        self.genome.clone()
    }
}

/// Overloading operators '==' and '!=' for Individual
impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl Eq for Individual {}

/// Overloading '<' for Individual
impl PartialOrd for Individual {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.fitness.cmp(&other.fitness))
    }
}

impl Ord for Individual {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fitness.cmp(&other.fitness)
    }
}
