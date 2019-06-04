use crate::math::{
    Vec3,
    Mat3,
    Quaternion,
};
use crate::Ray;

#[derive(Debug)]
pub struct Camera
{
    pub pos: Vec3,
    pub rot: Quaternion,
    pub tan_half_fov: f64,
    pub width: usize,
    pub height: usize,
    pub aspect: f64,
}

impl Camera
{
    pub fn new(pos: Vec3, rot: Quaternion, fov: f64, width: usize, height: usize) -> Camera
    {
        Camera {
            pos,
            rot,
            tan_half_fov: (fov.to_radians() * 0.5).tan(),
            width,
            height,
            aspect: width as f64 / height as f64
        }
    }

    pub fn look_at(pos: Vec3, target: Vec3, up: Vec3, fov: f64, width: usize, height: usize) -> Camera
    {
        let forward   = (target - pos).normalized();
        let right     = up.normalized().cross(forward).normalized();
        let camera_up = forward.cross(right);

        Camera {
            pos,
            rot: Quaternion::from_mat3(Mat3::from_col_vec3([
                right, camera_up, forward,
            ])).normalized(),
            tan_half_fov: (fov.to_radians() * 0.5).tan(),
            width,
            height,
            aspect: width as f64 / height as f64,
        }
    }

    pub fn width(&self) -> usize
    {
        self.width
    }

    pub fn height(&self) -> usize
    {
        self.height
    }

    pub fn set_w_h(&mut self, w_h: (usize, usize))
    {
        self.width = w_h.0;
        self.height = w_h.1;
        self.aspect = w_h.0 as f64 / w_h.1 as f64;
    }

    pub fn rays(&self) -> Vec<Ray>
    {
        let look_base  = self.rot * Vec3::new(0.0, 0.0, 1.0);
        let look_right = self.rot * Vec3::new(self.tan_half_fov * self.aspect, 0.0, 0.0);
        let look_down  = self.rot * Vec3::new(0.0, -self.tan_half_fov, 0.0);

        let look_right_step = look_right / (self.width  - 1) as f64 * 2.0;
        let look_down_step  = look_down  / (self.height - 1) as f64 * 2.0;

        let upper_left = look_base - look_right - look_down;

        let mut rays = Vec::with_capacity(self.width * self.height);

        for y in 0..self.height
        {
            for x in 0..self.width
            {
                // TODO optimize
                rays.push(Ray::new(
                    self.pos,
                    upper_left + look_right_step * x as f64 + look_down_step * y as f64
                ));
            }
        }

        rays
    }

    pub fn line_rays(&self, y: usize) -> Vec<Ray>
    {
        let look_base  = self.rot * Vec3::new(0.0, 0.0, 1.0);
        let look_right = self.rot * Vec3::new(self.tan_half_fov * self.aspect, 0.0, 0.0);
        let look_down  = self.rot * Vec3::new(0.0, -self.tan_half_fov, 0.0);

        let look_right_step = look_right / (self.width  - 1) as f64 * 2.0;
        let look_down_step  = look_down  / (self.height - 1) as f64 * 2.0;

        let upper_left = look_base - look_right - look_down;

        let mut rays = Vec::with_capacity(self.width);

        for x in 0..self.width
        {
            // TODO optimize
            rays.push(Ray::new(
                self.pos,
                upper_left + look_right_step * x as f64 + look_down_step * y as f64
            ));
        }

        rays
    }
}