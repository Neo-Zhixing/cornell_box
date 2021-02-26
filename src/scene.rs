use glam::Vec3;
use std::io::{Read};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Light {
    position: Vec3,
    diffuse: Vec3,
    specular: Vec3,
}

#[derive(Debug, Deserialize)]
pub struct Sphere {
    position: Vec3,
    radius: f32,
    diffuse: Vec3,
    specular: Vec3,
    ambient: Vec3,
    mirror: bool
}

#[derive(Debug, Deserialize)]
pub struct Quad {
    positions: [Vec3; 3],
    diffuse: Vec3,
    specular: Vec3,
    ambient: Vec3,
    is_diffuse: bool,
}

#[derive(Debug, Deserialize)]
pub struct Scene {
    shininess: f32,
    antialias: u32,
    background: Vec3,
    maxdepth: u32,
    resolution: (u32, u32),
    lights: Vec<Light>,
    spheres: Vec<Sphere>,
    quads: Vec<Quad>,
}

impl Scene {
    pub fn parse<T: Read>(input: T) -> ron::Result<Self> {
        return ron::de::from_reader(input)
    }
}