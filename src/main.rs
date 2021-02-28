use crate::renderer::render;
use image::codecs::tiff::TiffEncoder;
use std::fs::File;
use std::io::BufReader;

pub mod primitive;
pub mod renderer;
pub mod scene;

fn main() {
    //  Rendering images
    std::fs::create_dir("rendered");
    let file = File::open("./assets/cornell2.ron").unwrap();
    let buf_reader = BufReader::new(file);
    let scene = scene::Scene::parse(buf_reader).unwrap();
    let image = render(&scene);

    image.save("rendered/rendered2.tiff"); // Change this line for

    // Rendering videos
    /*

    std::fs::create_dir_all("rendered/video");
    let file = File::open("./assets/cornell1.ron").unwrap();
    let buf_reader = BufReader::new(file);
    let mut scene = scene::Scene::parse(buf_reader).unwrap();

    let p0 = scene.spheres[0].position;
    let p1 = scene.spheres[1].position;
    let p2 = scene.spheres[2].position;
    let p3 = scene.spheres[3].position;
    let l0 = scene.lights[0].position;
    for i in 0..600 {
        println!("Rendering frame {}", i);
        scene.spheres[0].position = p0;
        scene.spheres[0].position.x += (i as f32 / 30.0).sin() * 70.0;
        scene.spheres[0].position.y += (i as f32 / 45.0).cos() * 30.0;


        scene.spheres[1].position = p1;
        scene.spheres[1].position.x += (i as f32 / 90.0).sin() * 70.0;
        scene.spheres[1].position.y += (i as f32 / 45.0).sin() * 30.0;
        scene.spheres[1].position.z += (i as f32 / 60.0).cos() * 50.0;

        scene.spheres[2].position = p2;
        scene.spheres[2].position.x += (i as f32 / 90.0).sin() * 12.0;
        scene.spheres[2].position.y += (i as f32 / 45.0).sin() * 38.0;
        scene.spheres[2].position.z += (i as f32 / 60.0).cos() * 79.0;

        scene.spheres[3].position = p3;
        scene.spheres[3].position.x += (i as f32 / 29.0).sin() * 36.0;
        scene.spheres[3].position.y += (i as f32 / 58.0).sin() * 14.0;
        scene.spheres[3].position.z += (i as f32 / 47.0).cos() * 100.0;

        scene.lights[0].position = l0;
        scene.lights[0].position.x += (i as f32 / 78.0).sin() * 88.0;
        let image = render(&scene);
        image.save(format!("rendered/video/{}.tiff", i)); // Change this line for
    }
     */
}
