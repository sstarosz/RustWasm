use crate::hittable_object::HitRecord;
use crate::hittable_object::HittableObject;
use crate::ray::Ray;

pub struct HitableList<T: HittableObject>
{
    pub objects: Vec<T>,

}

impl<T: HittableObject> HitableList<T> {



    pub fn new() -> HitableList<T> {
        HitableList { objects: Vec::new()}
    }

    pub fn clear(mut self)
    {
        self.objects.clear();
    }

    pub fn add(&mut self,  object: T)
    {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool
    {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closet_so_far = t_max;

        for object in &self.objects
        {
            if object.hit(*ray, t_min, closet_so_far, &mut temp_rec)
            {
                hit_anything = true;
                closet_so_far = temp_rec.t;
                *rec = std::mem::replace(&mut temp_rec, HitRecord::default());
            }
        }

        return hit_anything;
    }

}