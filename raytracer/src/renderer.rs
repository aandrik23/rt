use crate::camera::Camera;
use crate::color::Color;
use crate::ray::Ray;
use crate::scene::Scene;

const SHADOW_BIAS: f64 = 0.001;

fn background_color(scene: &Scene, ray: &Ray) -> Color {
    // Vertical gradient sky: white at horizon -> light blue at top,
    // based on ray direction's Y component.
    let t = 0.5 * (ray.direction.normalize().y + 1.0);
    let horizon = Color::white();
    let sky = scene.background;
    horizon * (1.0 - t) + sky * t
}

fn trace_ray(ray: &Ray, scene: &Scene) -> Color {
    match scene.hit(ray, 0.001, f64::INFINITY) {
        None => background_color(scene, ray),
        Some(hit) => {
            let surface_color = hit.material.texture.color_at(hit.point, hit.normal);
            let mut color = surface_color * hit.material.ambient;

            for light in &scene.lights {
                let to_light = light.position - hit.point;
                let distance_to_light = to_light.length();
                let l = to_light.normalize();

                // Shadow check: cast ray from hit point (offset along normal) toward the light.
                let shadow_origin = hit.point + hit.normal * SHADOW_BIAS;
                let shadow_ray = Ray::new(shadow_origin, l);
                let in_shadow = scene
                    .hit(&shadow_ray, 0.001, distance_to_light)
                    .is_some();

                if in_shadow {
                    continue;
                }

                // Diffuse
                let diff = hit.normal.dot(l).max(0.0);
                let diffuse = surface_color * light.color * (diff * light.intensity);

                // Specular (Phong)
                let reflected = (-l).reflect(hit.normal);
                let view_dir = (ray.origin - hit.point).normalize();
                let spec_angle = reflected.dot(view_dir).max(0.0);
                let spec = spec_angle.powf(hit.material.shininess);
                let specular = light.color * (spec * light.intensity);

                color += diffuse + specular;
            }

            color
        }
    }
}

pub fn render(scene: &Scene, camera: &Camera, width: usize, height: usize) {
    // Print PPM header to stdout
    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    for j in (0..height).rev() {
        // top to bottom
        eprintln!("Rendering row {}/{}", height - j, height);
        for i in 0..width {
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;
            let ray = camera.get_ray(u, v);
            let color = trace_ray(&ray, scene);

            // Gamma correction (gamma 2): sqrt each channel
            let corrected = color.gamma_correct().clamp(0.0, 1.0);
            let r = (corrected.r * 255.0) as u8;
            let g = (corrected.g * 255.0) as u8;
            let b = (corrected.b * 255.0) as u8;

            println!("{} {} {}", r, g, b);
        }
    }
}
