use std::{path::Path, vec};
use image::{ImageBuffer};
use ray::compute_color_scale;
use rust_raytracing::Ray;
use rust_raytracing::Camera;
use rust_raytracing::write_color;
use rand::{thread_rng, Rng};
use world::Sphere;

mod camera;
mod ray;
mod world;


fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100; //for antialiasing

    let path = Path::new("image.png");
    let mut imgbuf = ImageBuffer::new(image_width, image_height);
    let camera = Camera::new(90.0, aspect_ratio, 1.0);

    let sphere = Sphere {
        center: vec![0.0,0.0,-1.0],
        radius: 0.5,
    };

    let mut rng = thread_rng(); 

    for (i, j, pixel) in imgbuf.enumerate_pixels_mut(){
        let sample_ray = |_x| -> Vec<f64> {
            // get the vector to scale the color array of the pixel
            // compute_coefficient_ray(i+rng,j+rng,height,width,camera,world)
            compute_color_scale(i as f64 + rng.gen_range(0.0..1.0), j as f64 + rng.gen_range(0.0..1.0), image_height, image_width, &camera, &sphere)

            // camera.get_ray(u_scale, v_scale).ray_color()
        };
        let vector_addition = {
            |acc: Vec<f64>, samp: Vec<f64>| {
                acc.iter().zip(samp.iter()).map(|(lhs,rhs)| lhs+rhs ).collect()
            }
        };
        let color_coef = (0..samples_per_pixel).map(sample_ray).reduce(vector_addition).unwrap();
        let pixel_array: [u8; 3] = write_color(color_coef, samples_per_pixel).try_into().unwrap();
        *pixel = image::Rgb(pixel_array);
    }

    imgbuf.save(path).unwrap();

}
