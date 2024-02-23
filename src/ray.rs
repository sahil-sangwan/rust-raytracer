use rand::Rng;
use crate::camera::{Camera};
use crate::world::{Sphere, HitRecord, Hittable, scatter};

pub fn color_scale_recursive<T: Hittable>(light_ray: &Ray, world: &Vec<T>, depth: u32, shadow_scale: f64, attenuator: Vec<f64>) -> Vec<f64> {
    if depth <= 0 {
        return vec![0.0,0.0,0.0];
    }
    let object_hit_processor = {
        |acc:Option<HitRecord>, elem: &T| 
        match elem.hit(&light_ray, 0.001, 10000.0) {
            Some(rec) => {
                match acc {
                    Some(prev_rec) => {
                        if prev_rec.t < rec.t {
                            Some(prev_rec)
                        } else {
                            Some(rec)
                        }
                    }
                    None => {
                        Some(rec)
                    }
                }
            }
            None => {
                acc
            }
        }
    };

    let ray_collision: Option<HitRecord> = world.iter().fold(None, object_hit_processor);
    match ray_collision {
        Some(rec) => {
            let scattered_ray: Option<Ray> = scatter(&rec, light_ray);
            // move the match into a function that also makes call to scatter function
            match scattered_ray {
                Some(ray) => {
                    let attenuation = rec.albedo;
                    let new_attenuator = attenuator.iter().zip(attenuation.iter()).map(|(x,y)| x * y).collect();
                    color_scale_recursive(&ray, world, depth-1,shadow_scale*0.5, new_attenuator)
                },
                None => {
                    vec![0.0,0.0,0.0]
                },
            }
        },
        None => {
            let t = 0.5*(&light_ray.direction[1]  + 1.0);
            let v1 = vec![0.5, 0.7, 1.0];
            let v2 = vec![1.0, 1.0, 1.0];
            let s = v1.iter().map(|x| x * t);
            let v = v2.iter().map(|x| x * (1.0-t));
            let color_scale_iter = s.zip(v).map(|(x,y)| (x + y) * shadow_scale);
            color_scale_iter.zip(attenuator.iter()).map(|(x,y)| x * y).collect()
        },
    }
}

pub fn compute_color_scale(width_scale:f64, height_scale:f64, depth: u32, camera: &Camera, world: &Vec<Sphere>) -> Vec<f64> {
    let x_direction = (width_scale-0.5) * camera.viewport_width;
    let y_direction = -(height_scale-0.5) * camera.viewport_height;
    let light_ray = Ray::new(vec![camera.x, camera.y, camera.z], vec![x_direction, y_direction, -1.0*camera.focal_length]);
    color_scale_recursive(&light_ray, world, depth, 1.0, vec![1.0,1.0,1.0])
}

pub fn sum(v1: &Vec<f64>, v2: &Vec<f64>) -> Vec<f64> {
    let pairwise_iter = v1.iter().zip(v2.iter());
    pairwise_iter.map(|(x, y)| x + y).collect()
}

pub fn l2_norm_squared(vector: &Vec<f64>) -> f64 {
    vector.iter().map(|x| x*x).sum::<f64>()
}

pub fn normalize(vector: &Vec<f64>) -> Vec<f64> {
    let vec_norm = l2_norm_squared(vector).sqrt();
    vector.iter().map(|x| x / vec_norm).collect()
}

pub fn dot(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    let pairwise_iter = v1.iter().zip(v2.iter());
    pairwise_iter.map(|(x, y)| x * y).sum()
}

pub fn negate_vector(v: &Vec<f64>) -> Vec<f64> {
    v.iter().map(|cmp| cmp * -1.0).collect()
}

pub fn random_unit_sphere_vector() -> Vec<f64> {
    let mut rng = rand::thread_rng();
    loop {
        let v = vec![rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)];
        if l2_norm_squared(&v) < 1.0 {
            return normalize(&v);
        }
    }
}

pub fn reflect(incoming: &Vec<f64>, normal: &Vec<f64>) -> Vec<f64> {
    let bounce_length = -2.0 * dot(&incoming,&normal);
    incoming.iter().zip(normal.iter()).map(|(x,y)| x + bounce_length * y).collect()
}

pub fn write_color(pixel_color_scale:Vec<f64>, samples_per_pixel:i64, gamma: u32) -> Vec<u8>{
    // Divide the color scale by the number of samples and apple to RGB component
    pixel_color_scale.iter().map(|x| (255.0 * (x / samples_per_pixel as f64).powf(1.0 / gamma as f64)) as u8).collect()
}

pub struct Ray {
    pub origin: Vec<f64>,
    pub direction: Vec<f64>,
}

impl Ray {
    pub fn new(origin: Vec<f64>, direction: Vec<f64>) -> Ray {
        Ray{
            origin: origin,
            direction: normalize(&direction),
        }
    }
}