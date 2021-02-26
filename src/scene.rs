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
    pub shininess: f32,
    pub antialias: u32,
    pub background: Vec3,
    pub max_depth: u32,
    pub resolution: (u32, u32),
    pub lights: Vec<Light>,
    pub spheres: Vec<Sphere>,
    pub quads: Vec<Quad>,
    pub camera: Camera,
}

#[derive(Debug, Deserialize)]
pub struct Camera {
    pub position: Vec3,
    pub near: f32,
    pub far: f32,
    pub fov: f32,
}

impl Scene {
    pub fn parse<T: Read>(input: T) -> ron::Result<Self> {
        return ron::de::from_reader(input)
    }
}