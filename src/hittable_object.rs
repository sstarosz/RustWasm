
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub point:  Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {

    pub fn set_face_noraml(mut self, ray:Ray, outward_normal:Vec3) -> ()
    {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {-outward_normal};
    }
    
}


pub trait HittableObject {
    fn hit(self, ray: Ray, t_min: f64, t_max: f64, hit_record: HitRecord) -> bool;
}