use crate::{
    Ray,
    math::Vec3,
    Material,
};

mod sphere;
pub use sphere::Sphere;

pub trait Object: std::fmt::Debug
{
    fn hit(&self, ray: Ray, ray_range: (f64, f64)) -> Option<HitRecord>;
}

#[derive(Debug)]
pub struct HitRecord
{
    pub offset: f64,
    pub normal: Vec3,
    pub material: Material,
}