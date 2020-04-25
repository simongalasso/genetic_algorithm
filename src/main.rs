extern crate rand;

use rand::Rng;
use std::str;

/* Entity ------------------------------------------------ */

#[derive(Clone)]
struct Entity {
    genes: Vec<u8>,
    fitness: f64,
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
        self.fitness = (score / target.len() as f64).powf(2.0) + 0.01;
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
    generation: i32
}

impl Population {
    fn new(target: &String, size: i32) -> Population {
        Population {
            mutation_rate: 0.01,
            entities: (0..size).map(|_x| Entity::new(target.len())).collect(),
            generation: 0
        }
    }

    fn generate(&mut self) {
        let mut new_entities: Vec<Entity> = Vec::new();
        let mut max_fitness: f64 = 0.0;

        for i in 0..self.entities.len() {
            if self.entities[i].fitness > max_fitness {
                max_fitness = self.entities[i].fitness;
            }
        }

        for _ in 0..self.entities.len() {
            let partner_a: Entity = self.pick_partner();
            let partner_b: Entity = self.pick_partner();
            let mut child: Entity = partner_a.crossover(partner_b);
            child.mutate(self.mutation_rate);
            new_entities.push(child);
        }
        self.entities = new_entities;
        self.generation += 1;
    }

    fn pick_partner(&mut self) -> Entity {
        let mut fitness_sum: f64 = 0.0;

        for i in 0..self.entities.len() {
            fitness_sum += self.entities[i].fitness;
        }

        for i in 0..self.entities.len() {
            self.entities[i].fitness /= fitness_sum; 
        }

        let mut r: f64 = rand::thread_rng().gen_range(0.0, 1.0);
        let mut index = 0;
        while r > 0.0 {
            r -= self.entities[index].fitness;
            index += 1;
        }
        return self.entities[index - 1].clone();
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
    let target: String = String::from("This string is the goal");
    let mut population: Population = Population::new(&target, 200);

    loop {
        for i in 0..population.entities.len() {
            population.entities[i].calc_fitness(&target);
        }
        population.evaluate();
        population.generate();
    }
}
