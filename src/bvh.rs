use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct BVHNode {
    aabb: AABB,
    left: Option<Box<BVHNode>>,
    right: Option<Box<BVHNode>>,
    object: Option<Box<dyn Hittable + Send + Sync>>,
}

impl BVHNode {
    fn build(objects: &mut Vec<Box<dyn Hittable + Send + Sync>>) -> Option<Box<BVHNode>> {
        if objects.is_empty() {
            return None;
        }

        let aabb = objects
            .iter()
            .fold(AABB::empty(), |acc, o| acc.union(&o.aabb()));

        if objects.len() == 1 {
            let obj = objects.pop().unwrap();
            return Some(Box::new(BVHNode {
                aabb,
                left: None,
                right: None,
                object: Some(obj),
            }));
        }

        let centroid_aabb = objects.iter().fold(AABB::empty(), |mut acc, o| {
            let c = o.aabb().center();
            acc.extend(c);
            acc
        });

        let axis = centroid_aabb.longest_axis();
        objects.sort_by(|a, b| {
            a.aabb()
                .center()
                .element(axis)
                .partial_cmp(&b.aabb().center().element(axis))
                .unwrap()
        });

        let mid = objects.len() / 2;
        let mut right = objects.split_off(mid);
        let left = Self::build(objects);
        let right_child = Self::build(&mut right);

        Some(Box::new(BVHNode {
            aabb,
            left,
            right: right_child,
            object: None,
        }))
    }

    pub fn from_objects(mut objects: Vec<Box<dyn Hittable + Send + Sync>>) -> Option<Box<Self>> {
        Self::build(&mut objects)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.hit(ray, t_min, t_max) {
            return None;
        }

        if let Some(obj) = &self.object {
            return obj.hit(ray, t_min, t_max);
        }

        let mut closest = None;

        if let Some(ref left) = self.left {
            if let Some(hit) = left.hit(ray, t_min, t_max) {
                closest = Some(hit);
            }
        }

        let t_max = closest.as_ref().map(|h| h.t).unwrap_or(t_max);

        if let Some(ref right) = self.right {
            if let Some(hit) = right.hit(ray, t_min, t_max) {
                match closest {
                    Some(ref existing) if hit.t < existing.t => {
                        closest = Some(hit);
                    }
                    None => {
                        closest = Some(hit);
                    }
                    _ => {}
                }
            }
        }

        closest
    }

    fn aabb(&self) -> AABB {
        self.aabb
    }
}

pub struct BVH {
    root: Option<Box<BVHNode>>,
}

impl BVH {
    pub fn from_objects(objects: Vec<Box<dyn Hittable + Send + Sync>>) -> Option<Self> {
        BVHNode::from_objects(objects).map(|root| BVH { root: Some(root) })
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.root.as_ref()?.hit(ray, t_min, t_max)
    }

    fn aabb(&self) -> AABB {
        self.root
            .as_ref()
            .map(|r| r.aabb())
            .unwrap_or_else(AABB::empty)
    }
}
