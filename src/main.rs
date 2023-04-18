use std::{path::Path, vec};
use image::ImageBuffer;
use rust_raytracing::Ray;
use rust_raytracing::Camera;
use rust_raytracing::write_color;
use rand::{thread_rng, Rng};


fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let origin = vec![0.0,0.0,0.0];
    let samples_per_pixel = 100;

    // Camera
    let path = Path::new("image.png");
    let mut imgbuf = ImageBuffer::new(image_width, image_height);
    
    let camera = Camera::new(60.0, aspect_ratio, 1.0);

    let mut rng = thread_rng();    

    for j in (0..image_height).rev(){
        for i in 0..image_width{
            let mut cumulative_color_vec = vec![0.0,0.0,0.0];
            for s in 0..samples_per_pixel {
                let u_scale = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
                let v_scale = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;
                let traced_ray = camera.get_ray(u_scale, v_scale);

                // TODO Adapt this
                let pixel_vector = traced_ray.ray_color();
                cumulative_color_vec[0] += pixel_vector[0];
                cumulative_color_vec[1] += pixel_vector[1];
                cumulative_color_vec[2] += pixel_vector[2];
            }
            let pixel_array: [u8; 3] = write_color(cumulative_color_vec, samples_per_pixel).try_into().unwrap();
            let pixel = imgbuf.get_pixel_mut(i, j);

            *pixel = image::Rgb(pixel_array);
            // let u_scale = i as f64 / (image_width - 1) as f64;
            // let v_scale = j as f64 / (image_height - 1) as f64;

            // let traced_ray = camera.get_ray(u_scale, v_scale);
            // let pixel_vector = traced_ray.ray_color();
            // let pixel_array = pixel_vector.try_into().unwrap();


            // let pixel = imgbuf.get_pixel_mut(i, j);
            // *pixel = image::Rgb(pixel_array);
        }
    }
    imgbuf.save(path).unwrap();

}
