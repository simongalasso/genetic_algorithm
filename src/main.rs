extern crate rand;

use rand::Rng;
use std::str;

/* Entity ------------------------------------------------ */

#[derive(Clone)]
struct Entity {
    genes: Vec<u8>,
    fitness: f64
}

impl Entity {
    fn new(length: usize) -> Entity {
        Entity {
            genes: (0..length).map(|_x| rand::thread_rng().gen_range(32, 123)).collect(),
            fitness: 0.0
        }
    }

    fn calc_fitness(&mut self, target: &String) {
        let mut score: f64 = 0.0;

        for i in 0..self.genes.len() {
            if self.genes[i] == target.as_bytes()[i] {
                score += 1.0;
            }
        }
        self.fitness = score / target.len() as f64;
    }

    fn crossover(&self, partner: Entity) -> Entity {
        let mut child: Entity = Entity::new(self.genes.len());
        let midpoint: usize = rand::thread_rng().gen_range(0, self.genes.len()) as usize;

        for i in 0..self.genes.len() {
            child.genes[i] = if i > midpoint { self.genes[i] } else { partner.genes[i] }
        }
        return child;
    }

    fn mutate(&mut self, mutation_rate: f64) {
        for i in 0..self.genes.len() {
            if rand::thread_rng().gen_range(0.0, 1.0) < mutation_rate {
                self.genes[i] = rand::thread_rng().gen_range(32, 123);
            }
        }
    }
}

/* Population -------------------------------------------- */

struct Population {
    mutation_rate: f64,
    entities: Vec<Entity>,
    mating_pool: Vec<Entity>,
    generation: i32
}

impl Population {
    fn new(target: &String, size: i32) -> Population {
        Population {
            mutation_rate: 0.01,
            entities: (0..size).map(|_x| Entity::new(target.len())).collect(),
            mating_pool: Vec::new(),
            generation: 0
        }
    }

    fn natural_selection(&mut self) {
        let mut max_fitness: f64 = 0.0;

        for i in 0..self.entities.len() {
            if self.entities[i].fitness > max_fitness {
                max_fitness = self.entities[i].fitness;
            }
        }

        self.mating_pool.clear();
        for i in 0..self.entities.len() {
            let scaled_fitness: f64 = self.entities[i].fitness / max_fitness;
            let n: i32 = (scaled_fitness * 100.0).floor() as i32;
            //println!("{} | {}", scaled_fitness, n);
            for _ in 0..n {
                self.mating_pool.push(self.entities[i].clone());
            }
        }
    }

    fn generate(&mut self) {
        for i in 0..self.entities.len() {
            let a: usize = rand::thread_rng().gen_range(0, self.mating_pool.len());
            let b: usize = rand::thread_rng().gen_range(0, self.mating_pool.len());
            let partner_a: Entity = self.mating_pool[a].clone();
            let partner_b: Entity = self.mating_pool[b].clone();
            let mut child: Entity = partner_a.crossover(partner_b);
            child.mutate(self.mutation_rate);
            self.entities[i] = child;
        }
        self.generation += 1;
    }

    fn evaluate(&self) {
        let mut max_fitness: f64 = 0.0;
        let mut best: usize = 0;
        let mut total_fitness: f64 = 0.0;

        for i in 0..self.entities.len() {
            if self.entities[i].fitness > max_fitness {
                max_fitness = self.entities[i].fitness;
                best = i;
            }
            total_fitness += self.entities[i].fitness;
        }
        let fitness_average: i32 = (total_fitness / self.entities.len() as f64 * 100.0) as i32;
        println!("gen: {} | av: {}% | result: {}", self.generation, fitness_average, str::from_utf8(&self.entities[best].genes).unwrap());
    }
}

/* Functions --------------------------------------------- */

fn main() {
    let target: String = String::from("Hello world !");
    let mut population: Population = Population::new(&target, 500);

    loop {
        for i in 0..population.entities.len() {
            population.entities[i].calc_fitness(&target);
        }
        population.evaluate();
        population.natural_selection();
        population.generate();
    }
}
