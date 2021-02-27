use rayon::prelude::*;
use glam::{Vec3A, Vec2, Vec3};
use crate::scene::Scene;
use image::{RgbaImage};
use crate::renderer::Ray;


pub type Pixel = image::Rgba<u8>;

pub fn raygen<F>(scene: &Scene, fragment: F) -> RgbaImage
    where F: Fn(Ray) -> Pixel + Sync {
    let (width, height) = scene.resolution;
    let num_pixels = width * height;
    let mut data: Vec<Pixel> = (0..num_pixels)
        .into_par_iter() // parallelism
        .map(|i| {
            let (width, height) = scene.resolution;
            let y = i / width;
            let x = i - y * width;
            let aspect_ratio = (width as f32) / (height as f32);

            let eye: Vec3A = scene.camera.position.into();
            // Gramm-Schmidt Orthonormalization
            let up: Vec3A = Vec3A::new(0.0, 1.0, 0.0);
            let l: Vec3A = scene.camera.direction.into();
            let l = l.normalize();
            let v: Vec3A= l.cross(up).normalize();
            let u: Vec3A = v.cross(l); // u should already be normalized

            let av = aspect_ratio * v;
            let dl = scene.camera.near * l;

            let ll: Vec3A = eye
                + scene.camera.near * l
                - aspect_ratio * v - u; // ray to the top left corner

            let pixel_ndc = Vec2::new((x as f32 + 0.5) / width as f32, (y as f32 + 0.5) / height as f32);

            let pixel = ll + 2.0 * av * pixel_ndc.x + 2.0 * u * pixel_ndc.y;
            let dir = (pixel - eye).normalize();
            let ray = Ray {
                origin: eye,
                dir
            };
            fragment(ray)
        })
        .collect();
    assert_eq!(data.len(), data.capacity());
    let ratio = std::mem::size_of::<Pixel>() / std::mem::size_of::<u8>();
    debug_assert_eq!(ratio, 4);
    let data = unsafe {
        let reintepreted_data = Vec::from_raw_parts(
            data.as_mut_ptr() as *mut u8,
            data.len() * ratio,
            data.capacity() * ratio,
        );
        std::mem::forget(data);
        reintepreted_data
    };

    let image = image::ImageBuffer::from_vec(width, height, data).unwrap();
    image
}


