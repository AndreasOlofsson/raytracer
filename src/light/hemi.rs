use crate::{
    RGB,
    math::Vec3,
};

pub struct Hemi
{
    pub direction: Vec3,
    pub color: RGB,
}

impl Hemi
{
    pub fn new(direction: Vec3, color: RGB) -> Hemi
    {
        Hemi {
            direction: direction.normalized(),
            color,
        }
    }
}