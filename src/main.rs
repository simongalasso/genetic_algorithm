extern crate rand;

use rand::Rng;
use std::io;

/* Entity ------------------------------------------------ */

#[derive(Clone)]
struct Entity {
    genes: Vec<u8>,
    fitness: f64
}

impl Entity {
    fn new(length: usize) -> Entity {
        Entity {
            genes: (0..length).map(|_x| rand::thread_rng().gen()).collect(),
            fitness: 0.0
        }
    }

    fn calc_fitness(&mut self, target: &String) {
        let mut score: i32 = 0;

        for i in 0..self.genes.len() {
            if (self.genes[i] == target.as_bytes()[i]) {
                score += 1;
            }
        }
        self.fitness = score as f64 / target.len() as f64;
    }
}

/* Population -------------------------------------------- */

struct Population {
    target: String,
    mutation_rate: f64,
    entities: Vec<Entity>
}

impl Population {
    fn new(target: &String, size: i32) -> Population {
        Population {
            target: target.clone(),
            mutation_rate: 0.01,
            entities: (0..size).map(|_x| Entity::new(target.len())).collect()
        }
    }

    /*fn natural_selection() {
        let mut mating_pool: ;

        let mut max_fitness: f64 = 0.0;
        for i in self.entities.len() {
            if (self.entities[i].fitness > max_fitness) {
                max_fitness = self.entities[i].fitness;
            }
        }

        for i in self.entities.len() {
            let fitness = map
        }
    }*/
}

/* Functions --------------------------------------------- */

fn main() {
    let target: String = String::from("Hello world !");
    let mut population: Population = Population::new(&target, 200);

    loop {
        for i in 0..population.entities.len() {
            population.entities[i].calc_fitness(&target);
        }
        //population.natural_selection();
    }
}
