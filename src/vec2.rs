use std::ops::{Add, AddAssign, Mul, Neg, Sub};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {

    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 {x, y}
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn dot(&self, rhs: Vec2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
    /* Cross product is not defined for the 2D case
    pub fn cross(&self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.
        }
    */
    pub fn normalize(&self) -> Vec2 {
        let inv_n = 1.0 / self.length();
        Vec2 {
            x: inv_n * self.x,
            y: inv_n * self.y,
        }
    }
}



impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {

    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl <T> Mul<T> for Vec2 
    where T:Into<f32>
{
    type Output = Vec2;

    fn mul(self, value: T) -> Vec2 {
        let scalar = value.into();
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
 

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2{
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}