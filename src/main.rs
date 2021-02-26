use std::fs::File;
use std::io::BufReader;
use crate::renderer::render;
use image::codecs::tiff::TiffEncoder;

pub mod scene;
pub mod renderer;

fn main() {
    let file = File::open("./assets/cornell2.ron").unwrap();
    let buf_reader = BufReader::new(file);
    let scene = scene::Scene::parse(buf_reader).unwrap();
    let image = render(&scene);

    image.save("rendered.tiff");
}
