use crate::vec3::{Point3, Vec3};


pub struct Ray {
    pub origin: Point3, 
    pub direction: Vec3,
}
impl Ray {
    pub fn new() -> Ray {
        Ray {
            origin: Point3::new(),
            direction: Vec3::new(),
        }
    }

    pub fn new_ray(o: Point3, d: Vec3) -> Ray {
        Ray {
            origin: o,
            direction: d,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        Point3::with_vec(self.origin + t*self.direction)
    }
}
