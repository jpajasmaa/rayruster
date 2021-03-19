use crate::ray;
use crate::vec3;



pub trait hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<hit_record>;
}

pub struct hit_record {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}
impl hit_record {
    pub fn new_hit(p: Point3, t: f64) -> hit_record {
        hit_record {
            p,
            normal: p,
            t,
        }
    }
}
