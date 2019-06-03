use crate::math::Vec3;
use crate::{
    Ray,
    Object,
    HitRecord,
    Material,
};

#[derive(Debug, Clone, Copy)]
pub struct Sphere
{
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere
{
    pub fn new(
        center: Vec3,
        radius: f64,
        material: Material,
    ) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Object for Sphere
{
    fn hit(&self, ray: Ray, ray_range: (f64, f64)) -> Option<HitRecord>
    {
        let oc = ray.origin - self.center;

        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * oc.dot(ray.dir);
        let c = oc.dot(oc) - self.radius.powi(2);
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > std::f64::EPSILON
        {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            let t = if t1 < ray_range.1 && t1 > ray_range.0 { t1 } else { t2 };

            if t < ray_range.1 && t > ray_range.0
            {
                return Some(HitRecord {
                    offset: t,
                    normal: (ray.point_at_dist(t) - self.center) / self.radius,
                    material: self.material,
                });
            }
        }

        None
    }
}