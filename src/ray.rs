use rust_raytracing::{Camera};

use crate::world::{Hittable, Sphere, HitRecord};

pub fn compute_color_scale(i:f64,j:f64,image_height:u32,image_width:u32, camera: &Camera, sphere: &Sphere) -> Vec<f64> {
    let u = i / (image_width - 1) as f64;
    let v = j / (image_height - 1) as f64;

    // compute this automatically when camera is created
    let theta = &camera.vfov;
    let h = (theta/2.0).tan() * &camera.focal_length;
    let viewport_height = 2.0 * h;
    let viewport_width = &camera.aspect_ratio * viewport_height;

    let x_direction = (u-0.5) * viewport_width;
    let y_direction = (v-0.5) * viewport_height;
    
    // replace origin with camera origin
    let light_ray = Ray::new(vec![0.0, 0.0, 0.0], vec![x_direction, y_direction, -1.0*&camera.focal_length]);
    let record = sphere.hit(&light_ray, 0.0, 10000.0);
    // NOTE: should pattern match here
    // for Some, run the code block in lib.rs lines 66-74
    // for None
    match record {
        Some(r) => {
            let rec: HitRecord = r.into();
            let dir = &light_ray.direction;
            let orig = &light_ray.origin;
            let position: Vec<f64> = orig.iter().zip(dir.iter()).map(|(x,y)| x + rec.t * y).collect();
            let position = vec![position[0], position[1], position[2]+1.0];
            let normal = unit_vector(&position);
            let color_scale = vec![0.5 * normal[0] + 1.0, 0.5 * normal[1] + 1.0, 0.5 * normal[2] + 1.0];
            return color_scale;
        },
        None => {
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



    //get hittables as iterator
    //hittables.iter().find(intersect?)
        //intersect?<T: Hittable>(obj: T, ray: Ray)
    //if intersection, compute the hit record (i.e. do the work of the current ray_color function)
    //run code: ray_color(hittable, ray) -> hit_record
}

pub struct Ray {
    pub origin: Vec<f64>,
    pub direction: Vec<f64>,
}

pub fn unit_vector(vector: &Vec<f64>) -> Vec<f64> {
    let vec_norm = vector.iter().map(|x| x*x).sum::<f64>().sqrt();
    return vector.iter().map(|x| x / vec_norm).collect();

}
pub fn dot(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    let pairwise_iter = v1.iter().zip(v2.iter());
    return pairwise_iter.map(|(x, y)| x * y).sum();
}

pub fn write_color(pixel_color:Vec<f64>, samples_per_pixel:i64) -> Vec<u8>{
    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;

    let r = pixel_color[0] * scale;
    let g = pixel_color[1] * scale;
    let b = pixel_color[2] * scale;

    // Write the translated [0,255] value of each color component.
    return vec![
        (256.0 * Ray::clamp(r, 0.0, 0.999)) as u8,
        (256.0 * Ray::clamp(g, 0.0, 0.999)) as u8,
        (256.0 * Ray::clamp(b, 0.0, 0.999)) as u8];
}

impl Ray {
    pub fn new(origin: Vec<f64>, direction: Vec<f64>) -> Ray {
        Ray{
            origin: origin,
            direction: unit_vector(&direction),
        }
    }
    fn hit_sphere(&self, center: Vec<f64>, radius: f64, ray: &Ray) -> f64 {
        let oc = ray.origin.iter().zip(center.iter()).map(|(x, y)| x - y).collect();
        let a = dot(&ray.direction, &ray.direction);
        let half_b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - radius*radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-half_b - discriminant.sqrt() ) / a;
        }
    }
    pub fn ray_color(&self) -> Vec<f64> {
        let t = self.hit_sphere(vec![0.0,0.0,-1.0], 0.5, &self);
        if t > 0.0 {
            // find self at t and subtract -1 from z component
            // NOTE: the +1 comes from the position of the viewport relative to the camera (at plane z=-1)
            let position: Vec<f64> = self.origin.iter().zip(self.direction.iter()).map(|(x,y)| x + t * y).collect();
            let position = vec![position[0], position[1], position[2]+1.0];
            let normal = unit_vector(&position);
            let color_scale = vec![0.5 * normal[0] + 1.0, 0.5 * normal[1] + 1.0, 0.5 * normal[2] + 1.0];
            return color_scale;
            // return color_scale.iter().map(|x| (x*256.0) as u8).collect();
        }
        let unit_direction = unit_vector(&self.direction);
        let t = 0.5*(unit_direction[1] + 1.0);
        let v1 = vec![0.5, 0.7, 1.0];
        let v2 = vec![1.0, 1.0, 1.0];
        let s = v2.iter().map(|x| x * t);
        let v = v1.iter().map(|x| x * (1.0-t));
        let color_scale = s.zip(v).map(|(x,y)| x + y);
        // return color_scale.map(|x| (x*256.0) as u8).collect();
        return color_scale.collect();
    }
    fn clamp(x: f64, min: f64, max: f64) -> f64 {
        if x < min { return min; }
        if x > max { return max; }
        return x;
    }
}