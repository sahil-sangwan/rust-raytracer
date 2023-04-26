use crate::camera::{Camera};

use crate::world::{Sphere, HitRecord, Hittable};

pub fn color_scale_recursive(light_ray: &Ray, world: &Vec<Sphere>, depth: u32, shadow_scale: f64) -> Vec<f64> {
    if depth <= 0 {
        return vec![0.0,0.0,0.0];
    }
    let object_hit_processor = {
        |acc:Option<HitRecord>, elem: &Sphere| 
        match elem.hit(&light_ray, 0.0, 10000.0) {
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
    
    let record: Option<HitRecord> = world.iter().fold(None, object_hit_processor);
    match record {
        Some(rec) => {
            let reflected_ray = Ray::new(rec.point_of_contact, rec.normal);
            color_scale_recursive(&reflected_ray, world, depth-1,shadow_scale*0.5)
            // let color_scale: Vec<f64> = vec![0.5 * (rec.normal[0] + 1.0), 0.5 * (rec.normal[1] + 1.0), 0.5 * (rec.normal[2] + 1.0)];
            // color_scale
        },
        None => {
            let t = 0.5*(&light_ray.direction[1] + 1.0);
            let v1 = vec![0.5, 0.7, 1.0];
            let v2 = vec![1.0, 1.0, 1.0];
            let s = v1.iter().map(|x| x * t);
            let v = v2.iter().map(|x| x * (1.0-t));
            let color_scale = s.zip(v).map(|(x,y)| x + y).collect();
            color_scale
        },
    }
}

pub fn compute_color_scale(width_scale:f64, height_scale:f64, depth: u32, camera: &Camera, world: &Vec<Sphere>) -> Vec<f64> {
    let x_direction = (width_scale-0.5) * camera.viewport_width;
    let y_direction = -(height_scale-0.5) * camera.viewport_height;
    let light_ray = Ray::new(vec![camera.x, camera.y, camera.z], vec![x_direction, y_direction, -1.0*camera.focal_length]);
    color_scale_recursive(&light_ray, world, depth, 1.0)
    
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