pub enum LifeIndicator {
    Alive,
    Dead,
}

pub struct Generation {
    world_map: Vec<Vec<LifeIndicator>>,
    size: usize,
}

impl Generation {

    pub fn new(world_map: Vec<Vec<LifeIndicator>>, size: usize) -> Generation {
        Generation {
            world_map: world_map,
            size: size,
        }
    }

    pub fn cell_alive(&self, x: usize, y: usize) -> bool {
        match &self.world_map[x][y] {
            &LifeIndicator::Alive => true,
            &LifeIndicator::Dead => false,
        }
    }

    pub fn print_world(&self) {
        for world_line in &self.world_map {
            for cell in world_line {
                self.print_cell(&cell);
            }
            println!("");
        }
        println!("");
    }

    fn print_cell(&self, cell: &LifeIndicator) {
        match cell {
            &LifeIndicator::Alive => print!("*"),
            &LifeIndicator::Dead => print!("_"),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

pub struct WorldRules;

impl WorldRules {
    pub fn should_be_alive(neighbours_count: usize, currently_exists: bool) -> bool {
        match currently_exists {
            true => !(neighbours_count < 2 || neighbours_count > 3),
            false => neighbours_count == 3,
        }
    }

    pub fn count_neighbours(generation: &Generation, i: isize, j: isize) -> usize {
        let mut existing_neighbours_count = 0 as usize;

        for x in (i-1)..(i+2) {
            for y in (j-1)..(j+2) {
                if x == i && y == j {
                    continue;
                }

                if is_cell_alive_with_shifting(generation, x, y) {
                    existing_neighbours_count += 1;
                }
            }
        }

        existing_neighbours_count
    }
}

fn is_cell_alive_with_shifting(generation: &Generation, mut x_coordinate: isize, mut y_coordinate: isize) -> bool {
    if x_coordinate < 0 {
        x_coordinate = generation.size() as isize - 1;
    } else if x_coordinate == generation.size() as isize {
        x_coordinate = 0;
    }

    if y_coordinate < 0 {
        y_coordinate = generation.size() as isize - 1;
    } else if y_coordinate == generation.size() as isize {
        y_coordinate = 0;
    }

    generation.cell_alive(x_coordinate as usize, y_coordinate as usize)
}

#[test]
fn test_generation_is_cell__alive() {
    let generation: Generation = Generation::new(vec![vec![LifeIndicator::Alive]], 1);
    assert!(generation.cell_alive(0, 0));
}

#[test]
fn test_generation_is_cell__alive() {
    let generation: Generation = Generation::new(vec![vec![LifeIndicator::Dead]], 1);
    assert!(generation.cell_alive(0, 0) == false);
}
