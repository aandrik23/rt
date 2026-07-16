mod camera;
mod color;
mod light;
mod material;
mod objects;
mod ray;
mod renderer;
mod scene;
mod texture;
mod vec3;

use renderer::render;

fn main() {
    let width = 800;
    let height = 600;

    // Scene selector via CLI arg
    // cargo run -- scene1 > scene1.ppm
    let args: Vec<String> = std::env::args().collect();
    let scene_name = args.get(1).map(|s| s.as_str()).unwrap_or("scene1");

    let (scene, camera) = scene::build(scene_name);

    render(&scene, &camera, width, height);
}
