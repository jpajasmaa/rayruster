use std::fmt;
use std::ops;


#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// type aliases
pub use Vec3 as Point3;
pub use Vec3 as Color;

#[allow(dead_code)]
impl Vec3 {
    // default is empty vector
    pub fn new() -> Vec3 {
        Vec3 {x:0.,y:0.,z:0.,}
    }

    pub fn new_vec(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x,y,z}
    }

    pub fn with_vec(rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x,
            y: rhs.y,
            z: rhs.z,
        }
    }

    pub fn length( &self ) -> f64 {
        self.length_squared().sqrt()
    }
    
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // vec3 utility functions
    #[inline]
    pub fn dot(u: &Vec3, v: &Vec3) -> f64{
        u.x * v.x + u.y * v.y + u.z * v.z
    } 
    #[inline]
    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3{
        Vec3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }
    #[inline]
    pub fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }

}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<&Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}
impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}
impl ops::Neg for Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// how to add assign
impl ops::AddAssign<&Vec3> for Vec3 {
    // lis??t????n itteen siksi mutable ite eik?? palauta mit????n
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x; 
        self.y += rhs.y; 
        self.z += rhs.z; 
    }
}
impl ops::MulAssign<f64> for Vec3 {
    // lis??t????n itteen siksi mutable ite eik?? palauta mit????n
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs; 
        self.y *= rhs; 
        self.z *= rhs; 
    }
}
impl ops::DivAssign<f64> for Vec3 {
    // lis??t????n itteen siksi mutable ite eik?? palauta mit????n
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs; 
        self.y /= rhs; 
        self.z /= rhs; 
    }
}


impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
