pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vfov: f64,
    pub aspect_ratio: f64,
    pub focal_length: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
}

impl Camera {
    pub fn new(origin: Vec<f64>, vfov: f64, aspect_ratio: f64, focal_length: f64) -> Camera {
        let vfov = vfov.to_radians();
        let visible_height = 2.0 * (vfov/2.0).tan() * focal_length;
        Camera { 
            x: origin[0],
            y: origin[1],
            z: origin[2],
            vfov: vfov,
            aspect_ratio: aspect_ratio,
            focal_length: focal_length,
            viewport_height: visible_height,
            viewport_width: aspect_ratio * visible_height,
        }
    }
}