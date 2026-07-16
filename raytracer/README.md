# RT — Ray Tracer

A ray tracer written in pure Rust (no external crates) that renders 3D scenes
containing spheres, cubes, flat planes, and cylinders to `.ppm` image files,
using Phong shading and shadows.

## How to Run

Output is written to stdout as PPM (P3, ASCII), so redirect it to a file:

```bash
cargo run -- scene1 > scene1.ppm
cargo run -- scene2 > scene2.ppm
cargo run -- scene3 > scene3.ppm
cargo run -- scene4 > scene4.ppm
cargo run -- scene5 > scene5.ppm
```

Running without an argument defaults to `scene1`. For faster iteration, use
a release build:

```bash
cargo run --release -- scene3 > scene3.ppm
```

Render progress is printed to stderr, so it won't corrupt the PPM output.

### Scenes

| Scene  | Contents                                              |
|--------|--------------------------------------------------------|
| scene1 | Sphere only                                             |
| scene2 | Plane + cube, lower brightness                          |
| scene3 | Sphere, cube, plane, and cylinder — full brightness      |
| scene4 | Same scene as scene3, viewed from a different camera     |
| scene5 | Same 4 objects, each with a procedural texture (checkerboard floor, striped cylinder, checkered cube, latitude/longitude sphere) |

## How to Create Each Object

```rust
// Sphere at position (1, 1, 1), radius 0.5
let s = Sphere {
    center: Vec3::new(1.0, 1.0, 1.0),
    radius: 0.5,
    material: Material::solid(Color::new(1.0, 0.2, 0.2), 32.0, 0.1),
};

// Flat plane (the ground), Y = 0, normal pointing up
let p = Plane {
    point: Vec3::new(0.0, 0.0, 0.0),
    normal: Vec3::new(0.0, 1.0, 0.0),
    material: Material::solid(Color::new(0.5, 0.5, 0.5), 8.0, 0.1),
};

// Cube from (0,0,-2) to (1,1,-1)
let c = Cube {
    min: Vec3::new(0.0, 0.0, -2.0),
    max: Vec3::new(1.0, 1.0, -1.0),
    material: Material::solid(Color::new(0.8, 0.5, 0.2), 16.0, 0.1),
};

// Cylinder at (0,-1,-3), radius 0.5, height 2
let cy = Cylinder {
    center: Vec3::new(0.0, -1.0, -3.0),
    radius: 0.5,
    height: 2.0,
    material: Material::solid(Color::new(0.2, 0.5, 0.8), 32.0, 0.1),
};
```

`Material::solid(color, shininess, ambient)` gives a flat, untextured
material. See the next section for patterned surfaces.

Add any object to a scene by boxing it into the `objects` vector:

```rust
let objects: Vec<Box<dyn Object>> = vec![Box::new(s), Box::new(p), Box::new(c), Box::new(cy)];
```

## How to Add Textures to a Surface

`Material.texture` is a `Texture` enum (in `texture.rs`) instead of a flat
color, so any object can have a patterned surface. Textures are computed
procedurally from the hit point / normal, so no image loading or extra
crates are needed.

```rust
// Flat color (equivalent to the old `albedo` field)
let flat = Material::solid(Color::new(0.6, 0.8, 0.3), 64.0, 0.1);

// 3D checkerboard — works on any object, tiled in world space
let checker = Material {
    texture: Texture::Checker {
        a: Color::new(0.9, 0.9, 0.9),
        b: Color::new(0.1, 0.1, 0.1),
        scale: 1.0, // size of each square
    },
    shininess: 8.0,
    ambient: 0.1,
};

// Horizontal stripes along Y (good for cylinders/cubes)
let stripes = Material {
    texture: Texture::Stripes {
        a: Color::new(0.2, 0.4, 0.8),
        b: Color::new(0.9, 0.9, 0.95),
        scale: 0.25, // stripe thickness
    },
    shininess: 32.0,
    ambient: 0.1,
};

// Latitude/longitude "beach ball" bands, mapped from the surface normal
// (so it doesn't stretch — best suited to spheres)
let grid = Material {
    texture: Texture::SphereGrid {
        a: Color::new(0.9, 0.2, 0.2),
        b: Color::new(0.95, 0.95, 0.9),
        lat_bands: 6.0,
        lon_bands: 12.0,
    },
    shininess: 32.0,
    ambient: 0.1,
};
```

Then assign it like any other material: `Sphere { center, radius, material: grid }`.
To add a new pattern, add a variant to the `Texture` enum in `texture.rs`
and implement its case in `Texture::color_at`. See `scene5` in `main.rs`
for a full worked example with all three patterns.

## How to Change Brightness

Brightness is controlled by a light's `intensity` field:

```rust
// Increase brightness
let light = PointLight { position: Vec3::new(3.0, 5.0, -2.0), color: Color::white(), intensity: 2.0 };

// Lower brightness
let light = PointLight { position: Vec3::new(3.0, 5.0, -2.0), color: Color::white(), intensity: 0.5 };
```

## How to Change Camera Position & Angle

```rust
// Overhead view
let camera = Camera {
    position: Vec3::new(0.0, 8.0, 0.0),  // high up on Y axis
    look_at:  Vec3::new(0.0, 0.0, -3.0), // looking down at the scene
    up: Vec3::new(0.0, 0.0, -1.0),       // forward as "up" when looking straight down
    fov: 60.0,
    aspect_ratio: 800.0 / 600.0,
};

// Side angle view (as in scene4)
let camera = Camera {
    position: Vec3::new(4.0, 3.0, 1.0),
    look_at:  Vec3::new(0.0, 0.0, -3.0),
    up: Vec3::new(0.0, 1.0, 0.0),
    fov: 60.0,
    aspect_ratio: 800.0 / 600.0,
};
```

## Shading Model

Each hit point is shaded with Phong lighting:

```
color = ambient_term + diffuse_term + specular_term
```

- **Surface color**: `material.texture.color_at(hit.point, hit.normal)` — a
  flat color or a procedural pattern (see "How to Add Textures" above)
- **Ambient**: `material.ambient * surface_color`
- **Diffuse**: `max(0, dot(normal, light_dir)) * surface_color * light.color * light.intensity`
- **Specular**: Phong reflection, `max(0, dot(reflect, view))^shininess * light.color * light.intensity`
  (specular highlights stay white/light-colored, independent of the texture)
- **Shadows**: a ray is cast from the hit point (offset slightly along the
  normal to avoid self-intersection) toward each light; if it hits another
  object before reaching the light, diffuse and specular are skipped for
  that light.

The sky background is a vertical gradient (white at the horizon fading to
light blue at the top), and final pixel colors are gamma-corrected
(gamma 2, i.e. `sqrt`) before being written out.

## Performance Notes

- Default resolution is **800×600**, set in `main.rs`.
- For faster local testing, temporarily lower `width`/`height` in `main.rs`
  (e.g. 200×150 or 400×300).
- Use `cargo run --release` for significantly faster renders.
- Render progress is logged to stderr as `Rendering row X/Y`.

## Project Structure

```
src/
├── main.rs           # Entry point: CLI arg parsing, dispatches to scene::build
├── vec3.rs            # Vec3 math (dot, cross, normalize, reflect)
├── ray.rs             # Ray { origin, direction }
├── camera.rs          # Look-at camera, get_ray(u, v)
├── color.rs            # Color (RGB f64), gamma correction, clamping
├── material.rs         # Material { texture, shininess, ambient }
├── texture.rs           # Texture enum: Solid, Checker, Stripes, SphereGrid
├── objects/
│   ├── mod.rs          # Object trait + HitRecord
│   ├── sphere.rs        # Sphere intersection
│   ├── plane.rs          # Plane intersection
│   ├── cube.rs            # Axis-aligned box (slab method)
│   └── cylinder.rs         # Finite cylinder (body + caps)
├── light.rs             # PointLight
├── scene.rs              # Scene { objects, lights, background } + 5 built-in demo scenes
└── renderer.rs            # Phong shading, shadows, PPM writer
```
