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

    /// Genrates a random vector uniformally distributed on a unit sphere.
    pub fn random_unit(rng: &mut rand::rngs::StdRng) -> Vec3
    {
        use rand::Rng;

        let theta = 2.0 * std::f64::consts::PI * rng.gen::<f64>();
        let phi   = std::f64::consts::PI * rng.gen::<f64>();

        let sin_phi = phi.sin();
        let cos_phi = phi.cos();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        Vec3 {
            x: sin_phi * cos_theta,
            y: sin_phi * sin_theta,
            z: cos_phi,
        }
    }

    /// Generates a random vector uniformally distributes on one half of a unit sphere.
    pub fn random_half_sphere(rng: &mut rand::rngs::StdRng, normal: Vec3) -> Vec3
    {
        let sphere = Vec3::random_unit(rng);
        let l = sphere.dot(normal);

        if l < 0.0
        {
            -sphere
        }
        else if l > 0.0
        {
            sphere
        }
        else
        {
            Vec3::random_half_sphere(rng, normal)
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

impl ops::Neg for Vec3
{
    type Output = Vec3;

    fn neg(self) -> Vec3
    {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}