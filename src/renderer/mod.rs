use crate::scene::Scene;
use image::{ImageBuffer, RgbImage, RgbaImage};

mod raygen;

pub fn render(scene: &Scene) -> RgbaImage {
    let image = raygen::raygen(scene, |ray| {
        image::Rgba([255, 255, 0, 255])
    });
    return image
}
