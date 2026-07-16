use crate::material::Material;
use crate::objects::{HitRecord, Object};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Plane {
    pub point: Vec3,  // any point on the plane
    pub normal: Vec3, // normalized
    pub material: Material,
}

impl Object for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denom = ray.direction.dot(self.normal);
        if denom.abs() < 1e-9 {
            return None; // parallel
        }

        let t = (self.point - ray.origin).dot(self.normal) / denom;
        if t <= t_min || t >= t_max {
            return None;
        }

        let point = ray.at(t);
        // Normal should face the incoming ray (so shading works from either side)
        let normal = if denom < 0.0 {
            self.normal
        } else {
            -self.normal
        };

        Some(HitRecord {
            t,
            point,
            normal,
            material: self.material,
        })
    }
}
