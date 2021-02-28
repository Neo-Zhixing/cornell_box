use crate::scene::Scene;
use image::{ImageBuffer, RgbImage, RgbaImage};
use glam::{Vec3A, Vec3};
use crate::primitive::Renderable;

mod raygen;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3A,
    pub dir: Vec3A,
}

pub struct Hit {
    normal: Vec3A,
    ambient: Vec3A,
    diffuse: Vec3A,
    specular: Vec3A,
    hitpoint: Vec3A,
    t: f32,
}

pub fn first_intersection(scene: &Scene, ray: Ray) -> Option<(Hit)> {
    let mut hit = Hit {
        normal: Vec3A::zero(),
        ambient: Vec3A::zero(),
        diffuse: Vec3A::zero(),
        specular: Vec3A::zero(),
        t: 0.0,
        hitpoint: Vec3A::zero(),
    };
    let mut z: f32 = f32::INFINITY; // 1 is farthest distance

    fn intersect<T: Renderable>(object: &T, ray: Ray, z: &mut f32, hit: &mut Hit) {
        let intersection = object.intersect(&ray);
        if intersection.is_none() {
            return;
        }
        let (t, normal) = intersection.unwrap();
        if t >= *z {
            return;
        }
        // regular shading
        *z = t;
        hit.normal = normal;
        hit.diffuse = object.get_diffuse().into();
        hit.ambient = object.get_ambient().into();
    }
    for sphere in scene.spheres.iter() {
        intersect(sphere, ray, &mut z, &mut hit);
    }
    for quad in scene.quads.iter() {
        intersect(quad, ray, &mut z, &mut hit);
    }
    if z == f32::INFINITY {
        None
    } else {
        hit.t = z;
        hit.hitpoint = (ray.origin + ray.dir * z).into();
        Some(hit)
    }
}

pub fn occluded(scene: &Scene, origin: Vec3A, destination: Vec3A) -> bool {
    let dir: Vec3A = destination - origin;
    let ray = Ray {
        origin: origin.into(),
        dir: dir.normalize(),
    };
    let max_t = dir.length();
    for sphere in scene.spheres.iter() {
        if let Some((t, _)) = sphere.intersect(&ray) {
            if t <= max_t {
                return true;
            }
        }
    }
    // We don't actually need to test for quads here
    return false;
}

pub fn render(scene: &Scene) -> RgbaImage {
    let image = raygen::raygen(scene, |ray| {
        let hit = first_intersection(scene, ray);
        if hit.is_none() {
            return scene.background;
        }
        let hit = hit.unwrap();

        let mut illumination: Vec3A = hit.ambient.into();
        for light in scene.lights.iter() {
            let light_position: Vec3A = light.position.into();
            if occluded(scene, hit.hitpoint + hit.normal * 0.01, light_position) {
                continue;
            }
            let light_diffuse_color: Vec3A = light.diffuse.into();
            let light_dir: Vec3A = light_position - hit.hitpoint;
            let light_dir: Vec3A = light_dir.normalize();
            let diffuse_shaded_color: Vec3A = hit.normal.dot(light_dir).abs() * hit.diffuse * light_diffuse_color;
            illumination += diffuse_shaded_color;
        }
        illumination /= (scene.lights.len() + 1) as f32;
        return illumination.into();
    });
    return image
}
