use crate::RGB;

#[derive(Debug, Clone, Copy)]
pub struct Material
{
    /// The color of the material.
    pub color: RGB,
    /// How reflective the material is.
    /// 
    /// 0.0 = completely diffuse
    /// 1.0 = completely reflective
    pub reflectivity: f32,
    /// Opacity
    pub opacity: f32,
    /// Refractive index
    /// Only applies if `opacity` < 1.0
    pub r_index: f32,
}

impl Material
{
    pub fn diffuse(color: RGB) -> Material
    {
        Material {
            color,
            reflectivity: 0.0,
            opacity: 1.0,
            r_index: 1.0,
        }
    }

    pub fn reflective(color: RGB, reflectivity: f32) -> Material
    {
        Material {
            color,
            reflectivity,
            opacity: 1.0,
            r_index: 1.0,
        }
    }

    pub fn transparent(
        color: RGB,
        reflectivity: f32,
        opacity: f32,
        r_index: f32
    ) -> Material {
        Material {
            color,
            reflectivity,
            opacity,
            r_index,
        }
    }
}