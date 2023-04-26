use crate::ray::{Ray, dot, unit_vector};

pub trait Hittable {
    fn hit(&self, ray: &Ray, min: f64, max: f64,) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point_of_contact: Vec<f64>,
    pub normal: Vec<f64>,
    pub t: f64,
    pub front_face: bool,
}

pub struct Sphere {
    pub center: Vec<f64>,
    pub radius: f64,
}

impl Sphere {
     fn get_root(min:f64, max:f64, half_b:f64, discriminant:f64, a:f64) -> Option<f64> {
        if discriminant < 0.0 {
            None
        } else if min < (-half_b - discriminant.sqrt()) / a  && max > (-half_b - discriminant.sqrt()) / a {
            Some((-half_b - discriminant.sqrt()) / a)
        } else if min < (-half_b + discriminant.sqrt()) / a  && max > (-half_b + discriminant.sqrt()) / a {
            Some((-half_b + discriminant.sqrt()) / a)
        } else {
            None
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self, ray: &Ray, min: f64, max: f64,) -> Option<HitRecord> {
        let oc = ray.origin.iter().zip(self.center.iter()).map(|(x, y)| x - y).collect();
        let a = dot(&ray.direction, &ray.direction);
        let half_b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - &self.radius*&self.radius;
        let discriminant = half_b*half_b - a*c;
        let root = Sphere::get_root(min,max,half_b,discriminant,a);
        match root {
            Some(r) => {
                let p: Vec<f64> = ray.origin.iter().zip(ray.direction.iter()).map(|(x,y)| x + r * y).collect();
                let n: Vec<f64> = p.iter().zip(self.center.iter()).map(|(x,y)| (x - y)/self.radius).collect();
                let n_unit = unit_vector(&n);
                return Some(HitRecord{
                    point_of_contact: p,
                    normal: n_unit,
                    t: r,
                    front_face: true,
                });  
            },
            None => {
                return None;
            },
        }
    }
}
