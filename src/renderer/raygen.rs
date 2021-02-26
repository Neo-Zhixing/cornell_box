use rayon::prelude::*;
use glam::{Vec3A, Vec2, Vec3};
use crate::scene::Scene;
use image::{RgbaImage};

pub struct Ray {
    pub origin: Vec3A,
    pub dir: Vec3A,
}

pub type Pixel = image::Rgba<u8>;

pub fn raygen<F>(scene: &Scene, fragment: F) -> RgbaImage
    where F: Fn(Ray) -> Pixel + Sync {
    let (width, height) = scene.resolution;
    let num_pixels = width * height;

    let aspect_ratio = (width as f32) / (height as f32);


    let mut data: Vec<Pixel> = (0..num_pixels)
        .into_par_iter() // parallelism
        .map(|i| {
            let y = i / width;
            let x = i - y * width;


            // x, y are in raster space now, indexed from top-left.
            // This is consistent with Vulkan.
            let pixel_ndc: Vec2 = Vec2::new((x as f32 + 0.5) / (width as f32), (y as f32 + 0.5) / (height as f32));

            // Screen space, (-1, -1) is at the left top corner
            let pixel_screen = 2.0 * pixel_ndc - Vec2::new(1.0f32, 1.0f32);

            let pixel_camera2d = Vec2::new(pixel_screen.x * aspect_ratio, pixel_screen.y) * ((scene.camera.fov / 180.0) * std::f32::consts::FRAC_PI_2).tan();
            let pixel_camera = Vec3A::new(pixel_camera2d.x, pixel_camera2d.y, -scene.camera.near);

            let ray = Ray {
                origin: scene.camera.position.into(),
                dir: pixel_camera.normalize(),
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


