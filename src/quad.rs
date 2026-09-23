use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

use crate::material::Material;
use std::sync::Arc;

pub struct Quad {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    d: Vec3,
    mat: Arc<dyn Material + Send + Sync>,
    normal: Vec3,
}

impl Quad {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, d: Vec3, mat: Arc<dyn Material + Send + Sync>) -> Self {
        let normal = (b - a).cross(&(c - a)).unit_vector();
        Self {
            a,
            b,
            c,
            d,
            mat,
            normal,
        }
    }

    fn hit_triangle(
        &self,
        r: &Ray,
        t_min: f64,
        t_max: f64,
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
    ) -> Option<HitRecord> {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let pvec = r.direction.cross(&edge2);
        let det = edge1.dot(&pvec);

        if det > -1e-8 && det < 1e-8 {
            return None;
        }

        let inv_det = 1.0 / det;
        let tvec = r.origin - v0;
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
        let front_face = r.direction.dot(&self.normal) < 0.0;

        Some(HitRecord {
            t,
            p,
            normal: self.normal,
            mat: Arc::clone(&self.mat),
            front_face,
        })
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let hit1 = self.hit_triangle(r, t_min, t_max, self.a, self.b, self.c);
        let hit2 = self.hit_triangle(r, t_min, t_max, self.a, self.c, self.d);

        match (hit1, hit2) {
            (Some(h1), Some(h2)) => Some(if h1.t < h2.t { h1 } else { h2 }),
            (Some(h), None) => Some(h),
            (None, Some(h)) => Some(h),
            (None, None) => None,
        }
    }

    fn aabb(&self) -> AABB {
        AABB::new(
            self.a.min(self.b).min(self.c).min(self.d),
            self.a.max(self.b).max(self.c).max(self.d),
        )
    }
}
