use std::fs::File;
use std::io::BufReader;

pub mod scene;


fn main() {
    let file = File::open("./assets/cornell2.ron").unwrap();
    let buf_reader = BufReader::new(file);
    let scene = scene::Scene::parse(buf_reader).unwrap();


}
