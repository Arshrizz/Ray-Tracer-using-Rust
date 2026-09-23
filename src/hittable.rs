use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::sync::Arc;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub mat: Arc<dyn Material + Send + Sync>,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn aabb(&self) -> AABB;
}
