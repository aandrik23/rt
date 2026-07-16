use crate::material::Material;
use crate::objects::{HitRecord, Object};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Cube {
    pub min: Vec3, // corner with smallest x,y,z
    pub max: Vec3, // corner with largest x,y,z
    pub material: Material,
}

impl Object for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin = [ray.origin.x, ray.origin.y, ray.origin.z];
        let dir = [ray.direction.x, ray.direction.y, ray.direction.z];
        let min = [self.min.x, self.min.y, self.min.z];
        let max = [self.max.x, self.max.y, self.max.z];

        let mut t_enter = t_min;
        let mut t_exit = t_max;
        let mut enter_axis = 0usize;
        let mut enter_sign = -1.0_f64;

        for axis in 0..3 {
            if dir[axis].abs() < 1e-12 {
                // Ray parallel to this slab; must already be within bounds
                if origin[axis] < min[axis] || origin[axis] > max[axis] {
                    return None;
                }
                continue;
            }

            let inv_d = 1.0 / dir[axis];
            let mut t_near = (min[axis] - origin[axis]) * inv_d;
            let mut t_far = (max[axis] - origin[axis]) * inv_d;
            let mut sign = -1.0_f64;
            if t_near > t_far {
                std::mem::swap(&mut t_near, &mut t_far);
                sign = 1.0;
            }

            if t_near > t_enter {
                t_enter = t_near;
                enter_axis = axis;
                enter_sign = sign;
            }
            if t_far < t_exit {
                t_exit = t_far;
            }

            if t_enter >= t_exit {
                return None;
            }
        }

        if t_enter <= t_min || t_enter >= t_max {
            return None;
        }

        let point = ray.at(t_enter);
        let mut normal = Vec3::zero();
        match enter_axis {
            0 => normal.x = enter_sign,
            1 => normal.y = enter_sign,
            _ => normal.z = enter_sign,
        }

        Some(HitRecord {
            t: t_enter,
            point,
            normal,
            material: self.material,
        })
    }
}
