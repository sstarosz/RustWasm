use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable_object::HittableObject;
use crate::hittable_object::HitRecord;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere{


    pub center: Vec3,
    pub radius: f64,
}


impl HittableObject for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, mut record: &mut HitRecord) -> bool {
        let oc :Vec3 = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0
        {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root 
        {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max > root
            {
                return  false;
            }
        }

        record.t = root;
        record.point = ray.at(record.t);
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_noraml(ray, outward_normal);

        return true;
    }
}



#[test]
fn test_sphere_creation() {
    let center = Vec3{x:0.0, y:0.0, z:0.0};
    let radius = 1.0;
    let sphere = Sphere{center: center, radius: radius};

    assert_eq!(sphere.center, center);
    assert_eq!(sphere.radius, radius);

}

#[test]
fn test_sphere_hit() {
    let sphere = Sphere {
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 1.0,
    };
    
    let ray = Ray{origin: Vec3::new(0.0, 0.0, -3.0), direction: Vec3::new(0.0, 0.0, 1.0)};
    let t_min = 0.0;
    let t_max = f64::INFINITY;
    let mut record = HitRecord::default();
    
    let hit = sphere.hit(ray, t_min, t_max, &mut record);
    
    assert!(hit);
    assert_eq!(record.t, 2.0);
    assert_eq!(record.point, Vec3::new(0.0, 0.0, -1.0));
    assert_eq!(record.normal, Vec3::new(0.0, 0.0, -1.0));
}