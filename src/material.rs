use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
    fn emitted(&self, _rec: &HitRecord) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere();

        if scatter_direction.length_squared() < 1e-8 {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = Vec3::reflect(&r_in.direction.unit_vector(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);

        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let (refraction_ratio, normal) = if rec.front_face {
            (1.0 / self.refraction_index, rec.normal)
        } else {
            (self.refraction_index, -rec.normal)
        };

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(&normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract
            || schlick(cos_theta, self.refraction_index) > rand::random::<f64>()
        {
            Vec3::reflect(&unit_direction, &normal)
        } else {
            Vec3::refract(&unit_direction, &normal, refraction_ratio)
        };

        Some((Ray::new(rec.p, direction), attenuation))
    }
}

fn schlick(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub struct Emissive {
    pub color: Vec3,
}

impl Emissive {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Material for Emissive {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Vec3)> {
        None
    }

    fn emitted(&self, _rec: &HitRecord) -> Vec3 {
        self.color
    }
}


pub struct HerringboneParquet {
    pub col_width: f64,
    pub plank_width: f64,
    pub fuzz: f64,
}

impl HerringboneParquet {
    pub fn new(col_width: f64, plank_width: f64, fuzz: f64) -> Self {
        Self {
            col_width: if col_width <= 0.0 { 0.36 } else { col_width },
            plank_width: if plank_width <= 0.0 { 0.09 } else { plank_width },
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }

    pub fn color_at(&self, p: &Vec3) -> Vec3 {
        // Rotate 90 degrees to align chevrons pointing toward left wall as in goal.jpeg
        let x = -p.z;
        let z = p.x;
        let col_w = self.col_width;
        let step_z = self.plank_width * 1.4142135623730951;

        let col = (x / col_w).floor() as i64;
        let u = x - (col as f64) * col_w;

        let z_shifted = z + if col % 2 != 0 { 0.5 * step_z } else { 0.0 };

        let d = if col % 2 == 0 {
            (z_shifted - u) / step_z
        } else {
            (z_shifted - (col_w - u)) / step_z
        };

        let pidx = d.floor() as i64;
        let v = d - (pidx as f64);

        let mut h = ((col as u64).wrapping_mul(374761393))
            ^ ((pidx as u64).wrapping_mul(668265263));
        h = (h ^ (h >> 13)).wrapping_mul(1274126177);
        let r_val = (h % 1000) as f64 / 1000.0;

        let len_coord = if col % 2 == 0 { u } else { col_w - u };
        let grain = ((len_coord * 95.0 + v * 25.0).sin() * 0.5 + 0.5) * 0.07;

        let edge_v = v.min(1.0 - v);
        let edge_u = (u / col_w).min(1.0 - (u / col_w));

        // Dark bevel groove between planks
        if edge_v < 0.035 || edge_u * col_w < 0.005 {
            Vec3::new(0.06, 0.02, 0.01)
        } else {
            // Rich golden-amber, honey and cognac teak wood tones
            let tone = 0.90 + (r_val - 0.5) * 0.45 + grain;
            Vec3::new(
                (0.56 * tone).clamp(0.02, 0.95),
                (0.24 * tone).clamp(0.01, 0.70),
                (0.05 * tone).clamp(0.00, 0.40),
            )
        }
    }
}

impl Material for HerringboneParquet {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let base_color = self.color_at(&rec.p);

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(&rec.normal).clamp(0.0, 1.0);
        let r0 = 0.14; // Higher specular varnish reflectivity
        let fresnel = r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);

        let mut rng = rand::thread_rng();
        if rng.r#gen::<f64>() < fresnel {
            let reflected = Vec3::reflect(&unit_direction, &rec.normal);
            let scattered = Ray::new(
                rec.p,
                reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            );
            if scattered.direction.dot(&rec.normal) > 0.0 {
                Some((scattered, Vec3::new(0.98, 0.96, 0.94)))
            } else {
                let mut scatter_dir = rec.normal + Vec3::random_in_unit_sphere();
                if scatter_dir.length_squared() < 1e-8 {
                    scatter_dir = rec.normal;
                }
                Some((Ray::new(rec.p, scatter_dir), base_color))
            }
        } else {
            let mut scatter_dir = rec.normal + Vec3::random_in_unit_sphere();
            if scatter_dir.length_squared() < 1e-8 {
                scatter_dir = rec.normal;
            }
            Some((Ray::new(rec.p, scatter_dir), base_color))
        }
    }
}

