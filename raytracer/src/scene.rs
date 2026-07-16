use crate::camera::Camera;
use crate::color::Color;
use crate::light::PointLight;
use crate::material::Material;
use crate::objects::cube::Cube;
use crate::objects::cylinder::Cylinder;
use crate::objects::plane::Plane;
use crate::objects::sphere::Sphere;
use crate::objects::{HitRecord, Object};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<PointLight>,
    pub background: Color,
}

impl Scene {
    // Returns the closest hit across all objects
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut result = None;
        for obj in &self.objects {
            if let Some(hit) = obj.hit(ray, t_min, closest) {
                closest = hit.t;
                result = Some(hit);
            }
        }
        result
    }
}

fn sky_color() -> Color {
    Color::new(0.5, 0.7, 1.0)
}

fn default_camera(position: Vec3, look_at: Vec3) -> Camera {
    Camera {
        position,
        look_at,
        up: Vec3::new(0.0, 1.0, 0.0),
        fov: 60.0,
        aspect_ratio: 800.0 / 600.0,
    }
}

fn sun() -> PointLight {
    PointLight {
        position: Vec3::new(3.0, 5.0, -2.0),
        color: Color::white(),
        intensity: 1.0,
    }
}

/// Built-in demo scenes, selected by name from `main`. Each returns the
/// scene plus the camera to render it with.
pub fn build(name: &str) -> (Scene, Camera) {
    match name {
        "scene2" => scene2_plane_and_cube(),
        "scene3" => scene3_all_objects(),
        "scene4" => scene4_all_objects_alt_camera(),
        "scene5" => scene5_textured(),
        _ => scene1_sphere(),
    }
}

// scene1: sphere only
fn scene1_sphere() -> (Scene, Camera) {
    let sphere = Sphere {
        center: Vec3::new(0.0, 0.0, -3.0),
        radius: 1.0,
        material: Material::solid(Color::new(0.6, 0.8, 0.3), 64.0, 0.1),
    };

    let scene = Scene {
        objects: vec![Box::new(sphere)],
        lights: vec![sun()],
        background: sky_color(),
    };
    let camera = default_camera(Vec3::new(0.0, 1.0, 1.0), Vec3::new(0.0, 0.0, -3.0));

    (scene, camera)
}

// scene2: plane + cube, lower brightness
fn scene2_plane_and_cube() -> (Scene, Camera) {
    let plane = Plane {
        point: Vec3::new(0.0, -1.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        material: Material::solid(Color::new(0.5, 0.5, 0.55), 16.0, 0.1),
    };
    let cube = Cube {
        min: Vec3::new(-1.0, -1.0, -4.0),
        max: Vec3::new(1.0, 1.0, -2.0),
        material: Material::solid(Color::new(0.8, 0.3, 0.2), 32.0, 0.1),
    };
    let light = PointLight {
        intensity: 0.5, // lower brightness
        ..sun()
    };

    let scene = Scene {
        objects: vec![Box::new(plane), Box::new(cube)],
        lights: vec![light],
        background: sky_color(),
    };
    let camera = default_camera(Vec3::new(0.0, 1.0, 3.0), Vec3::new(0.0, 0.0, -3.0));

    (scene, camera)
}

// scene3: sphere, cube, plane, and cylinder — full brightness
fn scene3_all_objects() -> (Scene, Camera) {
    let sphere = Sphere {
        center: Vec3::new(-2.0, 0.0, -4.0),
        radius: 1.0,
        material: Material::solid(Color::new(0.6, 0.8, 0.3), 64.0, 0.1),
    };
    let cube = Cube {
        min: Vec3::new(1.0, -1.0, -4.0),
        max: Vec3::new(2.5, 0.5, -2.5),
        material: Material::solid(Color::new(0.8, 0.5, 0.2), 32.0, 0.1),
    };
    let plane = Plane {
        point: Vec3::new(0.0, -1.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        material: Material::solid(Color::new(0.5, 0.5, 0.55), 16.0, 0.1),
    };
    let cylinder = Cylinder {
        center: Vec3::new(0.0, -1.0, -3.0),
        radius: 0.5,
        height: 2.0,
        material: Material::solid(Color::new(0.2, 0.5, 0.8), 32.0, 0.1),
    };

    let scene = Scene {
        objects: vec![
            Box::new(sphere),
            Box::new(cube),
            Box::new(plane),
            Box::new(cylinder),
        ],
        lights: vec![sun()],
        background: sky_color(),
    };
    let camera = default_camera(Vec3::new(0.0, 1.0, 1.0), Vec3::new(0.0, 0.0, -3.0));

    (scene, camera)
}

// scene4: same as scene3, viewed from a different camera
fn scene4_all_objects_alt_camera() -> (Scene, Camera) {
    let (scene, _) = scene3_all_objects();
    let camera = default_camera(Vec3::new(4.0, 3.0, 1.0), Vec3::new(0.0, 0.0, -3.0));

    (scene, camera)
}

// scene5: same 4 objects, each with a procedural texture
fn scene5_textured() -> (Scene, Camera) {
    let plane = Plane {
        point: Vec3::new(0.0, -1.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        material: Material {
            texture: Texture::Checker {
                a: Color::new(0.9, 0.9, 0.9),
                b: Color::new(0.1, 0.1, 0.1),
                scale: 1.0,
            },
            shininess: 8.0,
            ambient: 0.1,
        },
    };
    let sphere = Sphere {
        center: Vec3::new(-1.6, 0.0, -4.0),
        radius: 1.0,
        material: Material {
            texture: Texture::SphereGrid {
                a: Color::new(0.9, 0.2, 0.2),
                b: Color::new(0.95, 0.95, 0.9),
                lat_bands: 6.0,
                lon_bands: 12.0,
            },
            shininess: 32.0,
            ambient: 0.1,
        },
    };
    let cylinder = Cylinder {
        center: Vec3::new(0.8, -1.0, -4.0),
        radius: 0.6,
        height: 2.0,
        material: Material {
            texture: Texture::Stripes {
                a: Color::new(0.2, 0.4, 0.8),
                b: Color::new(0.9, 0.9, 0.95),
                scale: 0.25,
            },
            shininess: 32.0,
            ambient: 0.1,
        },
    };
    let cube = Cube {
        min: Vec3::new(2.2, -1.0, -4.6),
        max: Vec3::new(3.4, 0.2, -3.4),
        material: Material {
            texture: Texture::Checker {
                a: Color::new(0.8, 0.6, 0.1),
                b: Color::new(0.2, 0.15, 0.05),
                scale: 0.4,
            },
            shininess: 16.0,
            ambient: 0.1,
        },
    };

    let scene = Scene {
        objects: vec![
            Box::new(plane),
            Box::new(sphere),
            Box::new(cylinder),
            Box::new(cube),
        ],
        lights: vec![sun()],
        background: sky_color(),
    };
    let camera = default_camera(Vec3::new(0.5, 1.5, 1.5), Vec3::new(0.5, 0.0, -4.0));

    (scene, camera)
}
