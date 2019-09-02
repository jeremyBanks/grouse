///! The World contains the entire internal game state.
use crate::*;

#[derive(Debug)]
pub struct World {
    player: Player,
    level: Level,
}

impl World {
    pub fn new() -> World {
        World {
            level: Level::new(),
            player: Player::new(),
        }
    }

    pub fn rectangles(&self, width: f64, height: f64) -> Vec<Rectangle> {
        let mut rectangles = Vec::new();
        for terrain in self.level.terrain.iter() {
            rectangles.push(terrain.to_rectangle(width, height));
        }
        rectangles.push(self.player.to_rectangle(width, height));
        rectangles
    }

    pub fn tick(&mut self, dt: f64, input: &Input) {
        if input.acceleration.norm() > 0.0 {
            let acceleration = input.acceleration.normalize() * 4.0;
            self.player.velocity += dt * acceleration;
        }
        self.player.tick(dt);
    }
}

#[derive(Debug)]
pub struct Player {
    bottom_left: Vector,
    width_height: Vector,
    velocity: Vector,
}

impl Player {
    pub fn new() -> Player {
        Player {
            bottom_left: Vector::new(0.10, 0.10),
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

    fn tick(&mut self, dt: f64) {
        self.bottom_left += dt * self.velocity;
    }
}

#[derive(Debug)]
pub struct Level {
    terrain: Vec<Terrain>,
}

impl Level {
    pub fn new() -> Level {
        Level {
            terrain: vec![
                Terrain::new(Vector::new(0.00, 0.00), Vector::new(1.00, 0.05)),
                Terrain::new(Vector::new(0.00, 0.05), Vector::new(0.50, 0.05)),
            ],
        }
    }
}
