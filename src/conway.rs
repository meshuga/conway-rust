pub enum LifeIndicator {
    Alive,
    Dead,
}

pub struct Generation {
    world_map: Vec<Vec<LifeIndicator>>,
    size: i32,
}

impl Generation {

    pub fn new(world_map: Vec<Vec<LifeIndicator>>, size: i32) -> Generation {
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
    }

    fn print_cell(&self, cell: &LifeIndicator) {
        match cell {
            &LifeIndicator::Alive => print!("*"),
            &LifeIndicator::Dead => print!("_"),
        }
    }
}

#[test]
fn test_generation_is_cell__alive() {
    let generation: Generation = Generation::new(vec![vec![LifeIndicator::Alive]], 1);
    assert!(generation.cell_alive(0, 0));
}
