///! Static terrain in levels.
use crate::*;

#[derive(Debug)]
pub struct Terrain {
    bottom_left: Vector,
    width_height: Vector,
}

impl Terrain {
    pub fn new(bottom_left: Vector, width_height: Vector) -> Terrain {
        Terrain {
            bottom_left,
            width_height,
        }
    }
}

impl Physics for Terrain {
    fn bottom_left(&self) -> Vector {
        self.bottom_left
    }

    fn width_height(&self) -> Vector {
        self.width_height
    }

    fn physics_type(&self) -> PhysicsType {
        PhysicsType::Static
    }

    fn velocity(&self) -> Vector {
        Vector::zero()
    }
}
