use crate::material::Material;
use crate::objects::{HitRecord, Object};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let t1 = (-b - sqrt_d) / (2.0 * a);
        let t2 = (-b + sqrt_d) / (2.0 * a);

        let t = if t1 > t_min && t1 < t_max {
            t1
        } else if t2 > t_min && t2 < t_max {
            t2
        } else {
            return None;
        };

        let point = ray.at(t);
        let normal = (point - self.center).normalize();

        Some(HitRecord {
            t,
            point,
            normal,
            material: self.material,
        })
    }
}
