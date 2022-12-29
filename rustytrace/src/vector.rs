use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use std::fmt;

pub struct Vec3 {
    e: [f64; 3], // array of length 3 of f64 type
}

// Custom types for 3D points and Colors
pub type Point3 = Vec3;
pub type Color = Vec3;

// Provide Base functionality to the Vec3 struct
impl Vec3 {
    // Instantiate a new Vec3 struct
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }
}

// Implement the Indexing trait for the Vec3 struct
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

// Implement the IndexMut trait for the Vec3 struct
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

// Implement the Add trait for the Vec3 struct
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] * other, self[1] * other, self[2] * other],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Vec3 {
            e: [self[0] * other, self[1] * other, self[2] * other],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 { e: [self[0] * other[0], self[1] + other[1], self[2] + other[2]]

        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] / other, self[1] * other, self[2] * other],
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Vec3 {
            e: [self[0] / other, self[1] / other, self[2] / other],
        }
    }
}

// Utility Functions
// Collect the elements of the vector
pub fn x(self) -> f64 {
    self[0]
}

pub fn y(self) -> f64 {
    self[1]
}

pub fn z(self) -> f64 {
    self[2]
}

pub fn dot(self, other: Vec3) -> f64 {
    self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
}

pub fn length(self) -> f64 {
    self.dot(self).sqrt()
}

pub fn cross(self, other: Vec3) -> Vec3 {
    Vec3 {
        e: [
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0]
            ],
}

pub fn normalized(self) -> Vec3 {
    self / self.length()
}

// To Display the vec3 representation
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self[0], self[1], self[2])
    }
}