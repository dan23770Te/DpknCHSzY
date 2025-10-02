use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};
use serde::{Serialize, Deserialize};

// Define the chromosome structure, which will be used to represent solutions.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Chromosome {
    genes: Vec<f64>,
    fitness: f64,
}

// Define the population structure, which will manage the set of chromosomes.
struct Population {
    chromosomes: Vec<Chromosome>,
    size: usize,
    mutation_rate: f64,
    crossover_rate: f64,
}

impl Population {
    // Initialize a new population with a given size.
    fn new(size: usize, genes_count: usize) -> Self {
        let mut rng = thread_rng();
        let uniform = Uniform::from(0.0..1.0);
        let mut chromosomes = Vec::with_capacity(size);

        for _ in 0..size {
            let genes: Vec<f64> = (0..genes_count).map(|_| rng.sample(uniform)).collect();
            chromosomes.push(Chromosome { genes, fitness: 0.0 });
        }

        Population {
            chromosomes,
            size,
            mutation_rate: 0.01,
            crossover_rate: 0.7,
        }
    }

    // Evaluate the fitness of each chromosome in the population.
    fn evaluate_fitness(&mut self) {
        // Placeholder for fitness evaluation logic.
        for chromosome in &mut self.chromosomes {
            chromosome.fitness = self.calculate_fitness(&chromosome.genes);
        }
    }

    // Calculate the fitness of a chromosome (placeholder logic).
    fn calculate_fitness(&self, genes: &[f64]) -> f64 {
        // This is a placeholder for the actual fitness calculation.
        // In a real scenario, this would be a function that calculates
        // how well the chromosome solves the problem it's designed for.
        genes.iter().sum()
    }

    // Select the fittest chromosomes for the next generation.
    fn select(&self) -> Vec<&Chromosome> {
        let mut selected = Vec::with_capacity(self.size / 2);
        let mut sorted = self.chromosomes.clone();
        sorted.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        for i in 0..self.size / 2 {
            selected.push(&sorted[i]);
        }

        selected
    }

    // Perform crossover between selected chromosomes.
    fn crossover(&self, selected: &[&Chromosome]) -> Vec<Chromosome> {
        let mut offspring = Vec::with_capacity(self.size - self.chromosomes.len());
        let mut rng = thread_rng();
        let crossover_point = rng.gen_range(1..genes_count);

        for i in 0..selected.len() / 2 {
            let parent1 = selected[i].clone();
            let parent2 = selected[(selected.len() / 2) + i].clone();
            let child1 = Chromosome {
                genes: parent1.genes[..crossover_point].to_vec(),
                fitness: 0.0,
            };
            let child2 = Chromosome {
                genes: parent2.genes[..crossover_point].to_vec(),
                fitness: 0.0,
            };

            offspring.push(child1);
            offspring.push(child2);
        }

        offspring
    }

    // Mutate the offspring.
    fn mutate(&self, offspring: &mut Vec<Chromosome>) {
        for chromosome in offspring.iter_mut() {
            for gene in &mut chromosome.genes {
                if thread_rng().gen_bool(self.mutation_rate) {
                    *gene = thread_rng().gen_range(0.0..1.0);
                }
            }
        }
    }

    // Evolve the population to the next generation.
    fn evolve(&mut self) {
        self.evaluate_fitness();
        let selected = self.select();
        let offspring = self.crossover(&selected);
        self.mutate(&mut offspring);
        self.chromosomes = selected.into_iter().chain(offspring.into_iter()).take(self.size).collect();
    }
}

// A Rocket route to start the genetic algorithm.
#[get("/start")]
fn start_genetic_algorithm(pop: State<Population>) -> Json<Population> {
    pop.evolve();
    Json(pop)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![start_genetic_algorithm])
        .manage(Population::new(100, 10))
}
