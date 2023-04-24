use crate::camera::{Camera};

use crate::world::{Hittable, Sphere, HitRecord};

pub fn compute_color_scale(i:f64,j:f64,image_height:u32,image_width:u32, camera: &Camera, sphere: &Sphere) -> Vec<f64> {
    let u = i / (image_width - 1) as f64;
    let v = j / (image_height - 1) as f64;

    let x_direction = (u-0.5) * camera.viewport_width;
    let y_direction = (v-0.5) * camera.viewport_height;
    
    let light_ray = Ray::new(vec![camera.x, camera.y, camera.z], vec![x_direction, y_direction, -1.0*camera.focal_length]);
    let record: Option<HitRecord> = sphere.hit(&light_ray, 0.0, 10000.0);
    match record {
        Some(rec) => { // color the sphere
            let dir = &light_ray.direction;
            let orig = &light_ray.origin;
            let position: Vec<f64> = orig.iter().zip(dir.iter()).map(|(x,y)| x + rec.t * y).collect();
            let position = vec![position[0], position[1], position[2]+1.0];
            let normal = unit_vector(&position);
            let color_scale = vec![0.5 * normal[0] + 1.0, 0.5 * normal[1] + 1.0, 0.5 * normal[2] + 1.0];
            return color_scale;
        },
        None => { // fall back to background
            let unit_direction = unit_vector(&light_ray.direction);
            let t = 0.5*(unit_direction[1] + 1.0);
            let v1 = vec![0.5, 0.7, 1.0];
            let v2 = vec![1.0, 1.0, 1.0];
            let s = v2.iter().map(|x| x * t);
            let v = v1.iter().map(|x| x * (1.0-t));
            let color_scale = s.zip(v).map(|(x,y)| x + y).collect();
            return color_scale
        },
    };
}

pub fn unit_vector(vector: &Vec<f64>) -> Vec<f64> {
    let vec_norm = vector.iter().map(|x| x*x).sum::<f64>().sqrt();
    vector.iter().map(|x| x / vec_norm).collect()
}

pub fn dot(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    let pairwise_iter = v1.iter().zip(v2.iter());
    pairwise_iter.map(|(x, y)| x * y).sum()
}

pub fn write_color(pixel_color_scale:Vec<f64>, samples_per_pixel:i64) -> Vec<u8>{
    // Divide the color scale by the number of samples and apple to RGB component
    pixel_color_scale.iter().map(|x| (256.0 * x / samples_per_pixel as f64) as u8).collect()
}

pub struct Ray {
    pub origin: Vec<f64>,
    pub direction: Vec<f64>,
}

impl Ray {
    pub fn new(origin: Vec<f64>, direction: Vec<f64>) -> Ray {
        Ray{
            origin: origin,
            direction: unit_vector(&direction),
        }
    }
}