mod camera;
mod ray;
mod world;
use std::{path::Path, vec};
use image::{ImageBuffer};
use ray::{compute_color_scale, write_color};
use camera::Camera;
use rand::{thread_rng, Rng};
use world::Sphere;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100; //for antialiasing

    let path = Path::new("image.png");
    let mut imgbuf = ImageBuffer::new(image_width, image_height);
    let camera = Camera::new(vec![0.0,0.0,0.0], 90.0, aspect_ratio, 1.0);

    let sphere = Sphere {
        center: vec![0.0,0.0,-1.0],
        radius: 0.5,
    };

    let mut rng = thread_rng(); 

    for (i, j, pixel) in imgbuf.enumerate_pixels_mut(){
        // compute a ray sample at the pixel
        let sample_ray = |_x| -> Vec<f64> {
            compute_color_scale(i as f64 + rng.gen_range(0.0..1.0), j as f64 + rng.gen_range(0.0..1.0), image_height, image_width, &camera, &sphere)
        };
        // sum the ray samples
        let vector_addition = {
            |acc: Vec<f64>, samp: Vec<f64>| {
                acc.iter().zip(samp.iter()).map(|(lhs,rhs)| lhs+rhs ).collect()
            }
        };
        let color_coef = (0..samples_per_pixel).map(sample_ray).reduce(vector_addition).unwrap();
        // average the ray samples and multiply each scale component by 255 to get pixel color
        let pixel_array: [u8; 3] = write_color(color_coef, samples_per_pixel).try_into().unwrap();
        
        *pixel = image::Rgb(pixel_array);
    }

    imgbuf.save(path).unwrap();

}
