// useful link : https://www.geeksforgeeks.org/genetic-algorithms/

use std::env;

use genetic_algorithm::{Individual, IndividualTrait};
use rand::{thread_rng, Rng};

mod genetic_algorithm;

fn simulation(target: &str, population_size: &u32) {
    let target = target.as_bytes().to_vec();

    let mut population: Vec<Individual> = vec![];
    let mut generation: u32 = 1;

    // initiate the population
    (0..*population_size).for_each(|_| {
        let individual: Individual = Individual::new(&target);
        population.push(individual)
    });

    loop {
        println!(
            "Generation : {generation}\t String : {}\t Fitness Score : {}",
            String::from_utf8_lossy(&population[0].get_genome()),
            population[0].get_fitness()
        );
        generation += 1;

        // sort the population in increasing order of fitness score
        // the sort_unstable() method may not preserved the order of equal the elements
        population.sort_unstable();

        // if the individual having the lowest fitness score i.e
        // 0 then we know that we have reached to the target
        // and break the loop
        if population[0].get_fitness() <= 0 {
            println!("\nComputation finished");
            println!(
                "Generation : {generation}\t String : {}\t Fitness Score : {}",
                String::from_utf8_lossy(&population[0].get_genome()),
                population[0].get_fitness()
            );
            break;
        }

        //  if not generate new offspring for the next generation
        let mut new_population: Vec<Individual> = vec![];

        // Saving the 10% the best individuals of the population
        // the lower the fitness score is, the better this individual is
        // so the 10% the best individuals are on the beginning of the vector
        let size: u32 = (0.1f32 * *population_size as f32) as u32;
        (0..size).for_each(|i| {
            new_population.push(population[i as usize].clone());
        });

        // Generate the rest of the population based on the other 90%
        (size..*population_size).for_each(|_| {
            let parent1 = population[thread_rng().gen_range(0..=size) as usize].clone();
            let parent2 = population[thread_rng().gen_range(0..=size) as usize].clone();
            new_population.push(parent1.mate(&parent2));
        });

        // reset the population
        population = new_population.clone();
    }
}

fn main() {
    const POPULATION_SIZE: u32 = 100;
    let target: Vec<String> = env::args().collect();

    if target.len() > 1 {
        simulation(&target[1], &POPULATION_SIZE);
    } else {
        println!("Nothing to compute");
    }
}
