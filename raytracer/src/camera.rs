use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,          // typically Vec3(0,1,0)
    pub fov: f64,          // vertical field of view in degrees, e.g. 60.0
    pub aspect_ratio: f64, // width / height
}

impl Camera {
    // Compute the ray for pixel (u, v) where u,v are in [0,1]
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let forward = (self.look_at - self.position).normalize();
        let right = forward.cross(self.up).normalize();
        let up_cam = right.cross(forward);

        let viewport_height = 2.0 * (self.fov.to_radians() / 2.0).tan();
        let viewport_width = self.aspect_ratio * viewport_height;

        let lower_left = self.position + forward - right * (viewport_width / 2.0)
            - up_cam * (viewport_height / 2.0);

        let target =
            lower_left + right * (u * viewport_width) + up_cam * (v * viewport_height);
        let direction = (target - self.position).normalize();

        Ray::new(self.position, direction)
    }
}
