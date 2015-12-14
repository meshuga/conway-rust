use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io;

pub mod conway;
use conway::{Generation, LifeIndicator, WorldRules};

fn main() {
    let file_name = "initialMap.txt";
    let mut generation: Generation = match build_world(file_name) {
        Err(why) => panic!("Problem with parsing the file {}: {}",file_name, why ),
        Ok(generation) => generation,
    };
    generation.print_world();
    for _x in 0..120 {
        generation = build_new_generation(&generation);
        generation.print_world();
    }
}

fn build_world(file_name: &str) -> io::Result<Generation> {
    let path = Path::new(file_name);

    let mut file = try!(File::open(&path));

    let mut s = String::new();
    try!(file.read_to_string(&mut s));

    let world_map = s.split("\n")
        .map(|line| line.chars()
            .map(|character| match character {
                '*' => LifeIndicator::Alive,
                '_' => LifeIndicator::Dead,
                _ => panic!("Wrong cell value"),
            })
            .collect())
        .collect();

    Ok(Generation::new(world_map, 30))
}

fn build_new_generation(generation: &Generation) -> Generation {
    let world_map: Vec<Vec<LifeIndicator>> = (0..generation.size())
        .map(|i| (0..generation.size())
            .map(|j| {
                let count = WorldRules::count_neighbours(&generation, i as isize, j as isize);
                let currently_exists = generation.cell_alive(i, j);

                match WorldRules::should_be_alive(count, currently_exists) {
                    true => LifeIndicator::Alive,
                    false => LifeIndicator::Dead,
                }
            }).collect()
        ).collect();
    Generation::new(world_map, generation.size())
}
