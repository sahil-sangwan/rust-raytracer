mod camera;
mod ray;
mod world;
use std::{path::Path, vec, sync::Mutex};
use image::{ImageBuffer, Rgb};
use rayon::prelude::*;
use ray::{compute_color_scale, write_color};
use camera::Camera;
use rand;
use world::Sphere;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100; //for antialiasing
    let vfov = 90.0;
    let focal_length = 1.0;
    let recursion_depth: u32 = 50;
    let gamma: u32 = 2;

    let path = Path::new("image.png");
    let imgbuf: Mutex<ImageBuffer<Rgb<u8>, Vec<u8>>> = Mutex::new(ImageBuffer::new(image_width, image_height));
    let camera = Camera::new(vec![0.0,0.0,0.0], vfov, aspect_ratio, focal_length);

    let sphere = Sphere {
        center: vec![0.0,0.0,-1.0],
        radius: 0.5,
        albedo: vec![0.7, 0.3, 0.3],
        material: world::Material::Lambertian,
    };
    let bg_sphere = Sphere {
        center: vec![0.0, -100.5,-1.0],
        radius: 100.0,
        albedo: vec![0.8, 0.8, 0.8],
        material: world::Material::Lambertian,
    };
    let right_sphere = Sphere {
        center: vec![1.0,0.0,-1.0],
        radius: 0.5,
        albedo: vec![0.8, 0.6, 0.2],
        material: world::Material::Metal,
    };
    let left_sphere = Sphere {
        center: vec![-1.0,0.0,-1.0],
        radius: 0.5,
        albedo: vec![0.8, 0.8, 0.8],
        material: world::Material::Metal,
    };

    let world: Vec<Sphere> = vec![sphere, bg_sphere, right_sphere, left_sphere];

    (0..image_height).into_par_iter().rev().for_each(|j| {
        (0..image_width).into_par_iter().for_each(|i| {
            // compute a ray sample at the pixel
            let sample_ray = |_x| -> Vec<f64> {
                let width_scale = (i as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                let height_scale = (j as f64 + rand::random::<f64>()) / (image_height - 1) as f64;
                compute_color_scale(width_scale, height_scale, recursion_depth, &camera, &world)
            };
            // sum the ray samples
            let vector_addition = {
                |acc: Vec<f64>, samp: Vec<f64>| {
                    acc.iter().zip(samp.iter()).map(|(lhs,rhs)| lhs+rhs ).collect()
                }
            };
            let color_coef = (0..samples_per_pixel).into_par_iter().map(sample_ray).reduce(|| vec![0.0,0.0,0.0], vector_addition);
            // average the ray samples and multiply each scale component by 255 to get pixel color
            let pixel_array: [u8; 3] = write_color(color_coef, samples_per_pixel, gamma).try_into().unwrap();
            *imgbuf.lock().unwrap().get_pixel_mut(i, j) = image::Rgb(pixel_array);

        });
    });

    imgbuf.lock().unwrap().save(path).unwrap();

}
