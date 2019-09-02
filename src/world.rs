///! The World contains the entire internal game state.
use crate::*;

#[derive(Debug)]
pub struct World {
    player: Player,
    level: Level,
}

impl World {
    fn new() -> World {
        World {
            level: Level::new(),
            player: Player::new(),
        }
    }

    fn tick(&mut self, dt: f64, input: Input) {}
}

#[derive(Debug)]
pub struct Player {
    bottom_left: Vector,
    width_height: Vector,
    velocity: Vector,
}

impl Player {
    fn new() -> Player {
        Player {
            bottom_left: Vector::zero(),
            width_height: Vector::new(0.05, 0.10),
            velocity: Vector::zero(),
        }
    }
}

impl Physics for Player {
    fn physics_type(&self) -> PhysicsType {
        PhysicsType::Dynamic
    }

    fn bottom_left(&self) -> Vector {
        self.bottom_left
    }

    fn width_height(&self) -> Vector {
        self.width_height
    }

    fn velocity(&self) -> Vector {
        self.velocity
    }
}

#[derive(Debug)]
pub struct Level {
    terrain: Vec<Terrain>,
}

impl Level {
    fn new() -> Level {
        Level {
            terrain: vec![
                Terrain::new(Vector::new(0.00, 0.00), Vector::new(1.00, 0.05)),
                Terrain::new(Vector::new(0.00, 0.05), Vector::new(0.50, 0.05)),
            ],
        }
    }
}
