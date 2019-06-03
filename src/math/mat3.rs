use crate::math::Vec3;

use std::ops;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy)]
pub struct Mat3([f64; 9]);

impl Mat3
{
    pub fn from_values(values: [f64; 9]) -> Mat3
    {
        Mat3(values)
    }

    pub fn from_col_vec3(vecs: [Vec3; 3]) -> Mat3
    {
        Mat3([
            vecs[0].x,
            vecs[0].y,
            vecs[0].z,
            vecs[1].x,
            vecs[1].y,
            vecs[1].z,
            vecs[2].x,
            vecs[2].y,
            vecs[2].z,
        ])
    }

    pub fn from_row_vec3(vecs: [Vec3; 3]) -> Mat3
    {
        Mat3([
            vecs[0].x,
            vecs[1].y,
            vecs[2].z,
            vecs[0].x,
            vecs[1].y,
            vecs[2].z,
            vecs[0].x,
            vecs[1].y,
            vecs[2].z,
        ])
    }
}

impl ops::Index<usize> for Mat3
{
    type Output = [f64; 3];

    fn index(&self, row: usize) -> &[f64; 3]
    {
        (&self.0[(row * 3)..(row * 3 + 3)]).try_into().unwrap()
    }
}

impl ops::IndexMut<usize> for Mat3
{
    fn index_mut(&mut self, row: usize) -> &mut [f64; 3]
    {
        (&mut self.0[(row * 3)..(row * 3 + 3)]).try_into().unwrap()
    }
}