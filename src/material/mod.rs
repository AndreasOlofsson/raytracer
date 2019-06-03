use crate::RGB;

#[derive(Debug, Clone, Copy)]
pub struct Material
{
    pub color: RGB,
    pub reflectivity: f32,
}

impl Material
{
    pub fn diffuse(color: RGB) -> Material
    {
        Material {
            color,
            reflectivity: 0.0,
        }
    }
}