use glam::Vec3;
use std::io::{Read};
use serde::Deserialize;
use crate::primitive::{Sphere, Quad};



#[derive(Debug, Deserialize)]
pub struct Light {
    position: Vec3,
    diffuse: Vec3,
    specular: Vec3,
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
    pub direction: Vec3,
    pub far: f32,
    pub fov: f32,
}

impl Scene {
    pub fn parse<T: Read>(input: T) -> ron::Result<Self> {
        return ron::de::from_reader(input)
    }
}