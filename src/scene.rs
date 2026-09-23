use crate::hittable::Hittable;
use crate::material::{Dielectric, Emissive, HerringboneParquet, Lambertian, Material, Metal};
use crate::quad::Quad;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct SceneConfig {
    pub camera: CameraConfig,
    pub image: ImageConfig,
    pub materials: Vec<MaterialConfig>,
    pub objects: Vec<ObjectConfig>,
}

#[derive(Debug, Deserialize)]
pub struct CameraConfig {
    pub look_from: [f64; 3],
    pub look_at: [f64; 3],
    pub vfov: f64,
    pub aperture: f64,
    pub focus_dist: f64,
}

#[derive(Debug, Deserialize)]
pub struct ImageConfig {
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

#[derive(Debug, Deserialize, Default)]
pub struct MaterialConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub albedo: [f64; 3],
    #[serde(default)]
    pub fuzz: f64,
    #[serde(default)]
    pub refraction_index: f64,
    #[serde(default)]
    pub color: [f64; 3],
    #[serde(default)]
    pub col_width: f64,
    #[serde(default)]
    pub plank_width: f64,
}

#[derive(Debug, Deserialize)]
pub struct ObjectConfig {
    #[serde(rename = "type")]
    pub kind: String,
    pub center: [f64; 3],
    #[serde(default)]
    pub radius: f64,
    #[serde(default)]
    pub corners: Vec<[f64; 3]>,
    pub material: String,
}

impl SceneConfig {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let scene: SceneConfig = serde_json::from_str(&content)?;
        Ok(scene)
    }

    pub fn materials_map(&self) -> HashMap<String, Arc<dyn Material + Send + Sync>> {
        let mut map = HashMap::new();

        for material in &self.materials {
            let mat: Arc<dyn Material + Send + Sync> = match material.kind.as_str() {
                "lambertian" => Arc::new(Lambertian::new(Vec3::new(
                    material.albedo[0],
                    material.albedo[1],
                    material.albedo[2],
                ))),
                "metal" => Arc::new(Metal::new(
                    Vec3::new(material.albedo[0], material.albedo[1], material.albedo[2]),
                    material.fuzz,
                )),
                "dielectric" => Arc::new(Dielectric::new(if material.refraction_index == 0.0 {
                    1.5
                } else {
                    material.refraction_index
                })),
                "emissive" => Arc::new(Emissive::new(Vec3::new(
                    material.color[0],
                    material.color[1],
                    material.color[2],
                ))),
                "herringbone_parquet" => Arc::new(HerringboneParquet::new(
                    material.col_width,
                    material.plank_width,
                    if material.fuzz == 0.0 { 0.05 } else { material.fuzz },
                )),
                _ => Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
            };
            map.insert(material.name.clone(), mat);
        }

        map
    }

    pub fn build_objects(&self) -> Vec<Box<dyn crate::hittable::Hittable + Send + Sync>> {
        let materials = self.materials_map();
        let mut objects = Vec::new();

        for object in &self.objects {
            let mat = materials.get(&object.material).cloned().unwrap_or_else(|| {
                Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))
                    as Arc<dyn Material + Send + Sync>
            });

            let obj: Box<dyn Hittable + Send + Sync> = match object.kind.as_str() {
                "sphere" => Box::new(Sphere::new(
                    Vec3::new(object.center[0], object.center[1], object.center[2]),
                    object.radius,
                    mat,
                )),
                "quad" => Box::new(Quad::new(
                    Vec3::new(
                        object.corners[0][0],
                        object.corners[0][1],
                        object.corners[0][2],
                    ),
                    Vec3::new(
                        object.corners[1][0],
                        object.corners[1][1],
                        object.corners[1][2],
                    ),
                    Vec3::new(
                        object.corners[2][0],
                        object.corners[2][1],
                        object.corners[2][2],
                    ),
                    Vec3::new(
                        object.corners[3][0],
                        object.corners[3][1],
                        object.corners[3][2],
                    ),
                    mat,
                )),
                _ => Box::new(Sphere::new(
                    Vec3::new(object.center[0], object.center[1], object.center[2]),
                    object.radius,
                    mat,
                )),
            };
            objects.push(obj);
        }

        objects
    }
}
