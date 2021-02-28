use serde::Deserialize;
use glam::{Vec3, Vec3A};
use crate::renderer::Ray;

#[derive(Debug, Deserialize)]
pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub ambient: Vec3,
    pub mirror: bool
}

#[derive(Debug, Deserialize)]
pub struct Quad {
    pub positions: [Vec3; 3],
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub ambient: Vec3,
    pub is_diffuse: bool,
}


impl Sphere {

    // Returns t and surface normal
    pub(crate) fn intersect(&self, ray: &Ray) -> Option<(f32, Vec3)> {
        debug_assert!(ray.dir.is_normalized());
        let o: Vec3A = ray.origin.into();
        let d: Vec3A = ray.dir.into();

        let c: Vec3A = self.position.into();

        let b = d * (o - c);
        let b: f32 = 2.0 * (b.x + b.y + b.z);
        let c = (o - c).length_squared() - self.radius * self.radius;

        let discriminant: f32 = b*b - 4.0*c;
        if discriminant < 0.0 {
            return None;
        }
        let discriminant_sqrt = discriminant.sqrt();
        let mut t1 = -b - discriminant_sqrt;
        let mut t2 = -b + discriminant_sqrt;
        if t2 <= 0.0 {
            return None;
        }
        let hitpoint = ray.origin + t2 * ray.dir;
        let normal = hitpoint - self.position.into();
        Some((t2, normal.into()))
    }
}

impl Quad {
    pub(crate) fn intersect(&self, ray: &Ray) -> Option<f32> {
        debug_assert!(ray.dir.is_normalized());
        let a: Vec3A = self.positions[0].into();
        let b: Vec3A = self.positions[1].into();
        let c: Vec3A = self.positions[2].into();

        let p = b - a;
        let q = c - a;

        let tmp1 = ray.dir.cross(q);
        let dot1 = tmp1.dot(p);

        if dot1.abs() < 0.00001 {
            return None;
        }
        let f = 1.0 / dot1;
        let s = ray.origin - a;
        let u = f * s.dot(tmp1);
        if u < 0.0 || u > 1.0 {
            return None; // outside
        }
        let tmp2 = s.cross(p);
        let v = f * ray.dir.dot(tmp2);
        if v < 0.0 || v > 1.0 {
            return None; // outsize
        }
        let t = f * q.dot(tmp2);
        return Some(t);
    }
}