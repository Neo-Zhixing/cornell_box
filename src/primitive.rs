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

pub trait Renderable {
    fn get_diffuse(&self) -> Vec3;
    fn intersect(&self, ray: &Ray) -> Option<(f32, Vec3)>;
}

impl Renderable for Sphere {
    fn get_diffuse(&self) -> Vec3 {
        self.diffuse
    }
    // Returns t and surface normal
    fn intersect(&self, ray: &Ray) -> Option<(f32, Vec3)> {
        debug_assert!(ray.dir.is_normalized());

        let p: Vec3A = self.position.into();
        let m: Vec3A = ray.origin - p;
        let b = m.dot(ray.dir);
        let c = m.dot(m) - self.radius * self.radius;
        if c > 0.0 && b > 0.0 {
            return None;
        }
        let discr = b*b - c;
        if discr < 0.0 {
            return None;
        }
        let t = -b - discr.sqrt();
        let t = t.max(0.0);
        let hitpoint = ray.origin + t * ray.dir;
        let normal = hitpoint - p;
        Some((t, normal.into()))
    }
}

impl Renderable for Quad {
    fn get_diffuse(&self) -> Vec3 {
        self.diffuse
    }
    fn intersect(&self, ray: &Ray) -> Option<(f32, Vec3)> {
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
        let normal = p.cross(q);
        return Some((t, normal.into()));
    }
}