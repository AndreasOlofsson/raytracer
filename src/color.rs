use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct RGB
{
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl RGB
{
    pub fn new(r: f32, g: f32, b: f32) -> RGB
    {
        RGB {
            r, g, b,
        }
    }

    pub fn black() -> RGB
    {
        RGB {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn gray(value: f32) -> RGB
    {
        RGB {
            r: value,
            g: value,
            b: value,
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

    pub fn gamma(&self, gamma: f32) -> RGB
    {
        RGB {
            r: self.r.powf(gamma),
            g: self.g.powf(gamma),
            b: self.b.powf(gamma),
        }
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

impl ops::AddAssign for RGB
{
    fn add_assign(&mut self, other: RGB)
    {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
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
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl ops::MulAssign<f32> for RGB
{
    fn mul_assign(&mut self, other: f32)
    {
        self.r *= other;
        self.g *= other;
        self.b *= other;
    }
}

impl ops::Mul<RGB> for RGB
{
    type Output = RGB;

    fn mul(self, other: RGB) -> RGB
    {
        RGB {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl ops::Div<f32> for RGB
{
    type Output = RGB;

    fn div(self, other: f32) -> RGB
    {
        RGB {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}