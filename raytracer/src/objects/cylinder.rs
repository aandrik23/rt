use crate::material::Material;
use crate::objects::{HitRecord, Object};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Cylinder {
    pub center: Vec3, // center of the bottom cap
    pub radius: f64,
    pub height: f64,
    pub material: Material,
}

impl Object for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut best_t: Option<f64> = None;
        let mut best_normal = Vec3::zero();

        let y_bottom = self.center.y;
        let y_top = self.center.y + self.height;

        // --- Body (infinite cylinder along Y, clipped to height) ---
        let dx = ray.origin.x - self.center.x;
        let dz = ray.origin.z - self.center.z;
        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;

        if a > 1e-12 {
            let b = 2.0 * (dx * ray.direction.x + dz * ray.direction.z);
            let c = dx * dx + dz * dz - self.radius * self.radius;
            let discriminant = b * b - 4.0 * a * c;

            if discriminant >= 0.0 {
                let sqrt_d = discriminant.sqrt();
                for &t in &[(-b - sqrt_d) / (2.0 * a), (-b + sqrt_d) / (2.0 * a)] {
                    if t > t_min && t < t_max {
                        let hit_y = ray.origin.y + t * ray.direction.y;
                        if hit_y >= y_bottom && hit_y <= y_top {
                            if best_t.is_none() || t < best_t.unwrap() {
                                let hit_x = ray.origin.x + t * ray.direction.x;
                                let hit_z = ray.origin.z + t * ray.direction.z;
                                let normal = Vec3::new(
                                    (hit_x - self.center.x) / self.radius,
                                    0.0,
                                    (hit_z - self.center.z) / self.radius,
                                );
                                best_t = Some(t);
                                best_normal = normal;
                            }
                        }
                    }
                }
            }
        }

        // --- Caps ---
        if ray.direction.y.abs() > 1e-12 {
            for &(cap_y, normal_y) in &[(y_bottom, -1.0_f64), (y_top, 1.0_f64)] {
                let t = (cap_y - ray.origin.y) / ray.direction.y;
                if t > t_min && t < t_max {
                    let hit_x = ray.origin.x + t * ray.direction.x;
                    let hit_z = ray.origin.z + t * ray.direction.z;
                    let dist_sq = (hit_x - self.center.x).powi(2) + (hit_z - self.center.z).powi(2);
                    if dist_sq <= self.radius * self.radius {
                        if best_t.is_none() || t < best_t.unwrap() {
                            best_t = Some(t);
                            best_normal = Vec3::new(0.0, normal_y, 0.0);
                        }
                    }
                }
            }
        }

        best_t.map(|t| HitRecord {
            t,
            point: ray.at(t),
            normal: best_normal,
            material: self.material,
        })
    }
}
