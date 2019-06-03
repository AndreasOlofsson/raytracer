use crate::math::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray
{
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray
{
    pub fn new(origin: Vec3, dir: Vec3) -> Ray
    {
        Ray {
            origin,
            dir,
        }
    }

    pub fn point_at_dist(&self, dist: f64) -> Vec3
    {
        self.origin + self.dir * dist
    }

    pub fn reflect_at(&self, dist: f64, normal: Vec3) -> Ray
    {
        Ray {
            origin: self.point_at_dist(dist),
            dir: self.dir.reflect(normal),
        }
    }
}