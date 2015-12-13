use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub mod conway;

fn main() {
    let path = Path::new("initialMap.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => (),
    }

    let world_map = s.split("\n").map(|line| line.chars()
        .map(|character| match character {
            '*' => conway::LifeIndicator::Alive,
            '_' => conway::LifeIndicator::Dead,
            _ => panic!("Wrong cell value"),
        }).collect()
    ).collect();

    let generation = conway::Generation::new(world_map, 1);
    generation.print_world();
}
