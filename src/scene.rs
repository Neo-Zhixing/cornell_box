use crate::primitive::{Quad, Sphere};
use glam::Vec3;
use serde::Deserialize;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Light {
    pub position: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
}

#[derive(Debug, Deserialize)]
pub struct Scene {
    pub shininess: f32,
    pub antialias: u32,
    pub background: Vec3,
    pub max_depth: u8,
    pub resolution: (u32, u32),
    pub lights: Vec<Light>,
    pub spheres: Vec<Sphere>,
    pub quads: Vec<Quad>,
    pub camera: Camera,
}

#[derive(Debug, Deserialize)]
pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub fov: f32,
}

impl Scene {
    pub fn parse<T: Read>(input: T) -> ron::Result<Self> {
        return ron::de::from_reader(input);
    }
}
