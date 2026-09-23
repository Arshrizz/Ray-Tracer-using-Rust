mod aabb;
mod bvh;
mod camera;
mod cli;
mod hittable;
mod hittable_list;
mod material;
mod quad;
mod ray;
mod sampler;
mod scene;
mod sphere;
mod tone_mapping;
mod vec3;

use bvh::BVH;
use camera::Camera;
use clap::Parser;
use cli::Cli;
use hittable::Hittable;
use image::{ImageBuffer, RgbImage};
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rayon::prelude::*;
use scene::SceneConfig;
use std::time::Instant;
use vec3::Vec3;

fn background(ray: &crate::ray::Ray) -> Vec3 {
    let unit_dir = ray.direction.unit_vector();
    let t = 0.5 * (unit_dir.y + 1.0);
    // Soft realistic daytime sky gradient: warm horizon to clear sky
    (Vec3::new(1.0, 0.98, 0.95) * (1.0 - t) + Vec3::new(0.70, 0.82, 0.95) * t) * 0.85
}


fn ray_color(ray: &crate::ray::Ray, world: &dyn Hittable, depth: u32, rng: &mut StdRng) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        let emitted = rec.mat.emitted(&rec);

        if let Some((scattered, attenuation)) = rec.mat.scatter(ray, &rec) {
            // Russian roulette after a few bounces keeps deep paths cheap.
            let p = attenuation.max_component().min(0.95);
            if depth > 5 && p > 0.0 {
                if rng.r#gen::<f64>() > p {
                    return emitted;
                }
                let inverse = 1.0 / p;
                let bounced = ray_color(&scattered, world, depth - 1, rng);
                return emitted + (attenuation * inverse) * bounced;
            }

            let bounced = ray_color(&scattered, world, depth - 1, rng);
            return emitted + attenuation * bounced;
        }

        return emitted;
    }

    background(ray)
}

fn render_pixel(
    x: u32,
    y: u32,
    image_width: u32,
    image_height: u32,
    camera: &Camera,
    world: &BVH,
    samples: u32,
    max_depth: u32,
    seed: u64,
) -> [u8; 3] {
    let mut rng = StdRng::seed_from_u64(
        seed.wrapping_add((x as u64).wrapping_mul(0x9E3779B97F4A7C15))
            .wrapping_add((y as u64).wrapping_mul(0xBF58476D1CE4E5B9)),
    );

    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

    for _ in 0..samples {
        let s = (x as f64 + rng.r#gen::<f64>()) / (image_width as f64 - 1.0);
        let t = ((image_height - y) as f64 - rng.r#gen::<f64>()) / (image_height as f64 - 1.0);
        let ray = camera.get_ray(s, t, &mut rng);
        pixel_color = pixel_color + ray_color(&ray, world, max_depth, &mut rng);
    }

    let scale = 1.0 / samples as f64;
    let linear = pixel_color * scale;
    let mapped = tone_mapping::aces(linear);
    let [r, g, b] = tone_mapping::write_pixel(mapped);
    [r, g, b]
}

fn main() {
    let cli = Cli::parse();
    let started = Instant::now();

    let scene: SceneConfig = SceneConfig::load(&cli.scene)
        .or_else(|_| {
            if std::path::Path::new("output.json").exists() {
                SceneConfig::load("output.json")
            } else if std::path::Path::new("scenes/output.json").exists() {
                SceneConfig::load("scenes/output.json")
            } else {
                Ok(scene::SceneConfig::builtin())
            }
        })
        .expect("Failed to load scene");

    let image_width = cli.width.unwrap_or(scene.image.width);
    let image_height = cli.height.unwrap_or(scene.image.height);
    let samples = cli.samples.unwrap_or(scene.image.samples_per_pixel);
    let max_depth = cli.depth.unwrap_or(scene.image.max_depth);
    let seed = cli.seed;
    let aspect_ratio = image_width as f64 / image_height as f64;
    let camera = Camera::new(
        Vec3::new(
            scene.camera.look_from[0],
            scene.camera.look_from[1],
            scene.camera.look_from[2],
        ),
        Vec3::new(
            scene.camera.look_at[0],
            scene.camera.look_at[1],
            scene.camera.look_at[2],
        ),
        scene.camera.vfov,
        aspect_ratio,
        scene.camera.aperture,
        scene.camera.focus_dist,
    );

    let objects = scene.build_objects();
    let world = BVH::from_objects(objects).expect("Scene must contain at least one object");

    let pixels: Vec<u8> = (0..(image_width * image_height))
        .into_par_iter()
        .map(|index| {
            let x = index % image_width;
            let y = index / image_width;
            render_pixel(
                x,
                y,
                image_width,
                image_height,
                &camera,
                &world,
                samples,
                max_depth,
                seed,
            )
        })
        .flatten()
        .collect();

    let img: RgbImage =
        ImageBuffer::from_raw(image_width, image_height, pixels).expect("Invalid image size");
    img.save(&cli.output).expect("Failed to save image");

    println!(
        "Render complete: {}x{}, {} samples/pixel, max depth {}, output '{}'",
        image_width, image_height, samples, max_depth, cli.output
    );
    println!("Finished in {:.2?}.", started.elapsed());
}

impl SceneConfig {
    pub fn builtin() -> Self {
        Self {
            camera: scene::CameraConfig {
                look_from: [0.0, 0.0, 5.0],
                look_at: [0.0, 0.0, -1.0],
                vfov: 50.0,
                aperture: 0.0,
                focus_dist: 1.0,
            },
            image: scene::ImageConfig {
                width: 800,
                height: 450,
                samples_per_pixel: 100,
                max_depth: 50,
            },
            materials: vec![
                scene::MaterialConfig {
                    name: "ground".to_string(),
                    kind: "lambertian".to_string(),
                    albedo: [0.8, 0.8, 0.0],
                    ..Default::default()
                },
                scene::MaterialConfig {
                    name: "center".to_string(),
                    kind: "lambertian".to_string(),
                    albedo: [0.7, 0.3, 0.3],
                    ..Default::default()
                },
                scene::MaterialConfig {
                    name: "left".to_string(),
                    kind: "metal".to_string(),
                    albedo: [0.8, 0.8, 0.8],
                    fuzz: 0.3,
                    ..Default::default()
                },
                scene::MaterialConfig {
                    name: "right".to_string(),
                    kind: "metal".to_string(),
                    albedo: [0.8, 0.6, 0.2],
                    ..Default::default()
                },
            ],
            objects: vec![
                scene::ObjectConfig {
                    kind: "sphere".to_string(),
                    center: [0.0, -100.5, -1.0],
                    radius: 100.0,
                    corners: Vec::new(),
                    material: "ground".to_string(),
                },
                scene::ObjectConfig {
                    kind: "sphere".to_string(),
                    center: [0.0, 0.0, -1.0],
                    radius: 0.5,
                    corners: Vec::new(),
                    material: "center".to_string(),
                },
                scene::ObjectConfig {
                    kind: "sphere".to_string(),
                    center: [-1.0, 0.0, -1.0],
                    radius: 0.5,
                    corners: Vec::new(),
                    material: "left".to_string(),
                },
                scene::ObjectConfig {
                    kind: "sphere".to_string(),
                    center: [1.0, 0.0, -1.0],
                    radius: 0.5,
                    corners: Vec::new(),
                    material: "right".to_string(),
                },
            ],
        }
    }
}
