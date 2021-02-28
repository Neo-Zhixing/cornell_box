use crate::scene::Scene;
use image::{ImageBuffer, RgbImage, RgbaImage};
use glam::{Vec3A, Vec3};
use crate::primitive::Renderable;

mod raygen;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3A,
    pub dir: Vec3A,
}

pub fn render_object<T: Renderable>(object: &T, ray: &Ray, z: &mut f32, pixel: &mut Vec3) {
    let intersection = object.intersect(&ray);
    if intersection.is_none() {
        return;
    }
    let (t, normal) = intersection.unwrap();
    assert!(t > 0.0);
    if t >= *z {
        return;
    }
    // regular shading
    *z = t;
    *pixel = object.get_diffuse();
}

pub fn first_intersection(scene: &Scene, ray: Ray) -> Vec3 {
    let mut pixel: Vec3 = scene.background;
    let mut z: f32 = f32::INFINITY; // 1 is farthest distance
    for sphere in scene.spheres.iter() {
        render_object(sphere, &ray, &mut z, &mut pixel);
    }
    for quad in scene.quads.iter() {
        render_object(quad, &ray, &mut z, &mut pixel);
    }
    pixel
}

pub fn render(scene: &Scene) -> RgbaImage {
    let image = raygen::raygen(scene, |ray| {
        let pixel = first_intersection(scene, ray);
        let p = pixel * 255.0;
        return image::Rgba([
            p.x.round() as u8,
            p.y.round() as u8,
            p.z.round() as u8,
            255
        ]);
    });
    return image
}
