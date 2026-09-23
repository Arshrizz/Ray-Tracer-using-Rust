use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::sync::Arc;

pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    mat: Arc<dyn Material + Send + Sync>,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, mat: Arc<dyn Material + Send + Sync>) -> Self {
        Self { a, b, c, mat }
    }

    pub fn aabb(&self) -> AABB {
        let mut box_min = self.a.min(self.b).min(self.c);
        let mut box_max = self.a.max(self.b).max(self.c);
        // Add a tiny bit of padding to avoid zero-volume AABBs
        let padding = Vec3::new(0.0001, 0.0001, 0.0001);
        AABB::new(box_min - padding, box_max + padding)
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Möller–Trumbore algorithm
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;
        let pvec = r.direction.cross(&edge2);
        let det = edge1.dot(&pvec);

        if det > -1e-8 && det < 1e-8 {
            return None; // Ray is parallel to triangle
        }

        let inv_det = 1.0 / det;
        let tvec = r.origin - self.a;
        let u = tvec.dot(&pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(&edge1);
        let v = r.direction.dot(&qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = edge2.dot(&qvec) * inv_det;
        if t < t_min || t > t_max {
            return None;
        }

        let p = r.at(t);
        let mut normal = edge1.cross(&edge2).unit_vector();
        let front_face = r.direction.dot(&normal) < 0.0;
        if !front_face {
            normal = -normal;
        }

        Some(HitRecord { t, p, normal, mat: Arc::clone(&self.mat), front_face })
    }
}