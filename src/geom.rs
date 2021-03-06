pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Position {
        Position {x, y}
    }
}

pub fn clean_angle(angle: f64) -> f64 {
    use std::f64::consts::PI;
    (angle + 2. * PI) % (2. * PI)
}

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector{x, y}
    }

    pub fn dot(self, vect: Vector) -> f64 {
        self.x * vect.x + self.y * vect.y
    }
}