use std::ops;

pub use std::f32::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Vector { x, y }
    }

    pub fn zero() -> Self {
        Vector { x: 0.0, y: 0.0 }
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Vector> for Vector {
    fn add_assign(&mut self, other: Vector) {
        *self = *self + other;
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, other: Vector) {
        *self = *self - other;
    }
}

impl ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}
