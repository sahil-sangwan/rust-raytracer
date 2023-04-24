use crate::ray::{Ray, dot};

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
    pub fn get_root(min:f64, max:f64, half_b:f64, discriminant:f64, a:f64) -> f64 {
        if discriminant < 0.0 {
            -1.0
        } else if min < (-half_b - discriminant.sqrt()) / a  && max > (-half_b - discriminant.sqrt()) / a {
            (-half_b - discriminant.sqrt()) / a
        } else if min < (-half_b + discriminant.sqrt()) / a  && max > (-half_b + discriminant.sqrt()) / a {
            (-half_b + discriminant.sqrt()) / a
        } else {
            -1.0
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
        if root < 0.0 {
            None
        } else {
            let p: Vec<f64> = ray.origin.iter().zip(ray.direction.iter()).map(|(x,y)| x + root * y).collect();
            let n = p.iter().zip(ray.direction.iter()).map(|(x,y)| (x - y)/self.radius).collect();
            Some(HitRecord{
                point_of_contact: p,
                normal: n,
                t: root,
                front_face: true,
            })
        }
    }
}
