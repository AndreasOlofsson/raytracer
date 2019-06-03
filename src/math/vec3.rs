use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3
{
    pub fn zero() -> Vec3
    {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3
    {
        Vec3 {
            x, y, z,
        }
    }

    pub fn normalized(&self) -> Vec3
    {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();

        if length.abs() < std::f64::EPSILON
        {
            Vec3::zero()
        }
        else
        {
            Vec3 {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
            }
        }
    }

    pub fn dot(&self, other: Vec3) -> f64
    {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3
    {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3
    {
        *self - normal * (2.0 * self.dot(normal))
    }

    pub fn refract(&self, normal: Vec3, eta: f64) -> Vec3
    {
        let d = self.dot(normal);
        let k = 1.0 - eta * eta * (1.0 - d * d);

        if k < 1.0
        {
            Vec3::zero()
        }
        else
        {
            *self * eta - normal * (eta * d + k.sqrt())
        }
    }
}

impl ops::Add for Vec3
{
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3
    {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Vec3
{
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3
    {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f64> for Vec3
{
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3
    {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl ops::Div<f64> for Vec3
{
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3
    {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}