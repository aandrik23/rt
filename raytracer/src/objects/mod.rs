pub mod cube;
pub mod cylinder;
pub mod plane;
pub mod sphere;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub t: f64,       // distance along ray
    pub point: Vec3,  // hit point in world space
    pub normal: Vec3, // outward normal (normalized)
    pub material: Material,
}

pub trait Object: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
