use crate::scene::Scene;
use image::{ImageBuffer, RgbImage, RgbaImage};
use glam::{Vec3A, Vec3};

mod raygen;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3A,
    pub dir: Vec3A,
}

pub fn first_intersection(scene: &Scene, ray: Ray) -> Vec3 {
    let mut pixel: Vec3 = scene.background;
    let mut z: f32 = f32::INFINITY; // 1 is farthest distance
    for sphere in scene.spheres.iter() {
        let intersection = sphere.intersect(&ray);
        if intersection.is_none() {
            continue;
        }
        let (t, normal) = intersection.unwrap();
        assert!(t > 0.0);
        if t >= z {
            continue;
        }
        // regular shading
        pixel = sphere.diffuse / 255.0;
    }

    for quad in scene.quads.iter() {
        let intersection = quad.intersect(&ray);
        if intersection.is_none() {
            continue;
        }
        let t = intersection.unwrap();

        if t >= z {
            continue;
        }
        pixel = quad.diffuse / 255.0;
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
