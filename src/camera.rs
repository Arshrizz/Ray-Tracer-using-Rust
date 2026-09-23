use crate::ray::Ray;
use crate::sampler::random_in_unit_disk_rng;
use crate::vec3::Vec3;
use rand::rngs::StdRng;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let half_height = (focus_dist * (theta / 2.0).tan()).max(0.0001);
        let half_width = half_height * aspect_ratio;

        let w = (origin - look_at).unit_vector();
        // Standard camera basis: use world up (0,1,0) as the reference.
        let u = if w.cross(&Vec3::new(0.0, 1.0, 0.0)).length() > 1e-8 {
            Vec3::new(0.0, 1.0, 0.0).cross(&w).unit_vector()
        } else {
            // Camera looking straight up/down: fall back to +X as the up reference.
            Vec3::new(1.0, 0.0, 0.0).cross(&w).unit_vector()
        };
        let v = w.cross(&u);

        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - focus_dist * w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut StdRng) -> Ray {
        let disk = random_in_unit_disk_rng(rng);
        let offset = self.u * disk.x * self.lens_radius + self.v * disk.y * self.lens_radius;
        let ray_direction =
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset;
        Ray::new(self.origin + offset, ray_direction)
    }
}
