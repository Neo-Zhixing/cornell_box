use crate::primitive::Renderable;
use crate::scene::Scene;
use glam::{Vec3, Vec3A};
use image::{ImageBuffer, RgbImage, RgbaImage};

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
    mirrored: bool,
}

#[inline]
pub fn reflect_ray(dir: Vec3A, normal: Vec3A) -> Vec3A {
    dir - 2.0 * (dir.dot(normal)) * normal
}

pub fn first_intersection(scene: &Scene, ray: Ray) -> Option<(Hit)> {
    let mut hit = Hit {
        normal: Vec3A::zero(),
        ambient: Vec3A::zero(),
        diffuse: Vec3A::zero(),
        specular: Vec3A::zero(),
        t: 0.0,
        hitpoint: Vec3A::zero(),
        mirrored: false,
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
        hit.specular = object.get_specular().into();
        hit.mirrored = object.get_mirrored();
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

pub fn render_fragment(scene: &Scene, ray: Ray, depth: u8) -> Vec3A {
    let hit = first_intersection(scene, ray);
    if hit.is_none() {
        return scene.background.into();
    }
    let hit = hit.unwrap();
    let hitpoint_offset = hit.hitpoint + hit.normal * 0.1;

    let mut illumination: Vec3A = hit.ambient.into();
    for light in scene.lights.iter() {
        let light_position: Vec3A = light.position.into();
        if occluded(scene, hitpoint_offset, light_position) {
            continue;
        }
        let light_diffuse_color: Vec3A = light.diffuse.into();
        let light_specular_color: Vec3A = light.specular.into();
        let light_dir: Vec3A = light_position - hit.hitpoint;
        let light_dir: Vec3A = light_dir.normalize();
        let lambertian = hit.normal.dot(light_dir);
        let diffuse_shaded_color: Vec3A = lambertian * hit.diffuse * light_diffuse_color;

        let reflect_dir = reflect_ray(-light_dir, hit.normal);
        let spec_angle = reflect_dir.dot(ray.dir);
        let specular = spec_angle.powf(scene.shininess);
        let specular_shaded_color: Vec3A = hit.specular * specular * light_specular_color;

        illumination += diffuse_shaded_color + specular_shaded_color;
    }
    illumination /= (scene.lights.len()) as f32;

    if depth == 0 || !hit.mirrored {
        return illumination;
    }
    // mirrored surfaces are spheres
    let reflected_ray = Ray {
        origin: hitpoint_offset,
        dir: reflect_ray(-ray.dir, hit.normal).normalize(),
    };
    illumination = (illumination + render_fragment(scene, reflected_ray, depth - 1)) / 2.0;
    return illumination;
}

pub fn render(scene: &Scene) -> RgbaImage {
    let image = raygen::raygen(scene, |ray| {
        render_fragment(scene, ray, scene.max_depth).into()
    });
    return image;
}
