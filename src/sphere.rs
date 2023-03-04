use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hittable_object::HittableObject;
use crate::hittable_object::HitRecord;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere{


    pub center: Vec3,
    pub radius: f64,
}

impl Sphere
{
    fn new(center: Vec3, radius: f64) -> Sphere{
        Sphere{center,radius}
    }
}

impl HittableObject for Sphere {
    fn hit(self, ray: Ray, t_min: f64, t_max: f64, mut record: HitRecord) -> bool {
        let oc :Vec3 = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if (discriminant < 0.0)
        {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);

        let mut root = (-half_b - sqrtd) / a;
        if (root < t_min || t_max < root) 
        {
            root = (-half_b + sqrtd) / a;
            if ( root < t_min || t_max > root )
            {
                return  false;
            }
        }

        record.t = root;
        record.point = ray.at(record.t);
        record.normal = (record.point - self.center) / self.radius;
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_noraml(ray, outward_normal);

        return false;
    }
}



#[test]
fn test_sphere_creation() {
    let center = Vec3{x:1.0, y:2.0, z:3.0};
    let radius = 4.0;
    let sphere = Sphere::new(center, radius);

    assert_eq!(sphere.center, center);
    assert_eq!(sphere.radius, radius);
}