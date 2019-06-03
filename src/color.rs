use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct RGB
{
    r: f32,
    g: f32,
    b: f32,
}

impl RGB
{
    pub fn black() -> RGB
    {
        RGB {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn new(r: f32, g: f32, b: f32) -> RGB
    {
        RGB {
            r, g, b,
        }
    }

    pub fn clamp(&self) -> RGB
    {
        RGB {
            r: self.r.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
        }
    }

    pub fn as_u8(&self) -> [u8; 3]
    {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        ]
    }
}

impl ops::Add for RGB
{
    type Output = RGB;

    fn add(self, other: RGB) -> RGB
    {
        RGB {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl ops::Sub for RGB
{
    type Output = RGB;

    fn sub(self, other: RGB) -> RGB
    {
        RGB {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl ops::Mul<f32> for RGB
{
    type Output = RGB;

    fn mul(self, other: f32) -> RGB
    {
        RGB {
            r: self.r * other,
            g: self.r * other,
            b: self.r * other,
        }
    }
}
