use crate::math::Vec3;
use crate::math::Mat3;

use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Quaternion
{
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Quaternion
{
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Quaternion
    {
        Quaternion {
            x, y, z, w
        }
    }

    pub fn from_axis_rot(axis: Vec3, angle: f64) -> Quaternion
    {
        let basis = axis * (angle / 2.0).sin();

        Quaternion {
            x: basis.x,
            y: basis.y,
            z: basis.z,
            w: (angle / 2.0).cos(),
        }
    }

    pub fn from_mat3(m: Mat3) -> Quaternion
    {
        let trace = m[0][0] + m[1][1] + m[2][2];

        if trace > 0.0
        {
            let s = 2.0 * (trace + 1.0).sqrt();

            Quaternion {
                x: (m[2][1] - m[1][2]) / s,
                y: (m[0][2] - m[2][0]) / s,
                z: (m[1][0] - m[0][1]) / s,
                w: 0.25 * s,
            }
        }
        else
        {
            if m[0][0] > m[1][1] && m[0][0] > m[2][2]
            {
                let s = 2.0 * (1.0 + m[0][0] - m[1][1] - m[2][2]).sqrt();

                Quaternion {
                    x: 0.25 * s,
                    y: (m[0][1] - m[1][0]) / s,
                    z: (m[0][2] - m[2][0]) / s,
                    w: (m[2][1] - m[1][2]) / s,
                }
            }
            else if m[1][1] > m[2][2]
            {
                let s = 2.0 * (1.0 + m[1][1] - m[2][2] - m[0][0]).sqrt();

                Quaternion {
                    x: (m[0][1] - m[1][0]) / s,
                    y: 0.25 * s,
                    z: (m[1][2] - m[2][1]) / s,
                    w: (m[0][2] - m[2][0]) / s,
                }
            }
            else
            {
                let s = 2.0 * (1.0 + m[2][2] - m[0][0] - m[1][1]).sqrt();

                Quaternion {
                    x: (m[0][2] - m[2][0]) / s,
                    y: (m[1][2] - m[2][1]) / s,
                    z: 0.25 * s,
                    w: (m[1][0] - m[0][1]) / s,
                }
            }
        }
    }

    pub fn conjugate(&self) -> Quaternion
    {
        Quaternion {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    pub fn normalized(&self) -> Quaternion
    {
        let norm = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt();

        Quaternion {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
            w: self.w / norm,
        }
    }
}

impl ops::Mul for Quaternion
{
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Quaternion
    {
        Quaternion {
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y + self.y * other.w + self.z * other.x - self.x * other.z,
            z: self.w * other.z + self.z * other.w + self.x * other.y - self.y * other.x,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }
}

impl ops::Mul<Vec3> for Quaternion
{
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3
    {
        let quat = self * Quaternion::new(other.x, other.y, other.z, 0.0) * self.conjugate();
        
        Vec3::new(quat.x, quat.y, quat.z)
    }
}