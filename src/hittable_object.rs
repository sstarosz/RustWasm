
use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitRecord {
    pub point:  Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {

    pub fn set_face_noraml(&mut self, ray:Ray, outward_normal:Vec3) -> ()
    {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;

        if self.front_face
        {
            self.normal = outward_normal;
        }
        else {
            self.normal = -outward_normal;
        }
    }
    
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            point: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            normal: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            t: 0.0,
            front_face: false,
        }
    }
    
}


pub trait HittableObject {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record:  &mut HitRecord) -> bool;
}