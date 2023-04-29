use crate::ray::{Ray, dot, normalize, negate_vector, reflect};

pub trait Hittable {
    fn hit(&self, ray: &Ray, min: f64, max: f64,) -> Option<HitRecord>;
}

pub enum Material {
// incident ray, normal, point of contact, color attenuation vector
    // Metal(Ray, Vec<f64>, Vec<f64>, Vec<f64>),
    Metal(Vec<f64>),
    Lambertian,
}

pub fn scatter(mat: Material, hit_record: HitRecord, incident_ray: Ray) -> Option<Ray> {
    match mat {
        Material::Metal(albedo) => {
            let reflected: Vec<f64> = reflect(&incident_ray.direction, &hit_record.normal);
            let scattered = Ray::new(hit_record.point_of_contact, reflected);
            let attenuation = albedo;
            if dot(&scattered.direction, &hit_record.normal) > 0.0 {
                // return scattered ray and accumulate attenuation vector 
                Some(scattered)
            } else {
                None
            }
        },
        Material::Lambertian => {
            todo!()
        },
    }
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
    pub mat: Material
}

impl Sphere {
    pub fn get_root(&self, min:f64, max:f64, ray: &Ray) -> Option<f64> {
        let oc = ray.origin.iter().zip(self.center.iter()).map(|(x, y)| x - y).collect();
        let a = dot(&ray.direction, &ray.direction);
        let half_b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - &self.radius*&self.radius;
        let discriminant = half_b*half_b - a*c;
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
        let root = self.get_root(min,max,ray);
        match root {
            Some(r) => {
                let p: Vec<f64> = ray.origin.iter().zip(ray.direction.iter()).map(|(x,y)| x + r * y).collect();
                let n: Vec<f64> = p.iter().zip(self.center.iter()).map(|(x,y)| (x - y)/self.radius).collect();
                let n_unit = normalize(&n);
                if dot(&n_unit, &ray.direction) < 0.0{
                    Some(HitRecord{
                        point_of_contact: p,
                        normal: n_unit,
                        t: r,
                        front_face: true,
                    })
                } else {
                    Some(HitRecord{
                        point_of_contact: p,
                        normal: negate_vector(&n_unit),
                        t: r,
                        front_face: false,
                    })
                }  
            },
            None => {
                None
            },
        }
    }
}
