use rust_raytracing::Ray;

pub struct Camera {
    vfov: f64,
    aspect_ratio: f64,
    focal_length: f64,
}

impl Camera {
    pub fn new(vfov: f64, aspect_ratio: f64, focal_length: f64) -> Camera {
        Camera { 
            vfov: vfov.to_radians(),
            aspect_ratio: (aspect_ratio),
            focal_length: (focal_length) 
        }
    }
    pub fn get_ray(&self, u: f64, v:f64) -> Ray {
        let theta = &self.vfov;
        let h = (theta/2.0).tan() * &self.focal_length;
        let viewport_height = 2.0 * h;
        let viewport_width = &self.aspect_ratio * viewport_height;

        let x_direction = (u-0.5) * viewport_width;
        let y_direction = (v-0.5) * viewport_height;
        
        return Ray::new(vec![0.0, 0.0, 0.0], vec![x_direction, y_direction, -1.0*&self.focal_length]);
    }
}