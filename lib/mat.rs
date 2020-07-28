#[allow(unused_imports)]
use std::ops::{Add, Deref, Div, Mul, Sub};

use ndarray::prelude::*;
use ndarray::Array;
// use ndarray_linalg;

#[cfg(test)]
use ndarray_rand::rand_distr::Uniform;
#[cfg(test)]
use ndarray_rand::RandomExt;

#[allow(unused_imports)]
use std::f32;
use std::f64;

#[allow(unused_imports)]
use std::ops::Index;
#[allow(unused_imports)]
use std::ops::IndexMut;

use constants::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct Mat2x1([f64; 2]);

#[derive(Copy, Clone, Debug, Default)]
pub struct Mat1x2([f64; 2]);

#[derive(Copy, Clone, Debug, Default)]
pub struct Mat3x1([f64; 3]);

#[derive(Copy, Clone, Debug, Default)]
pub struct Mat1x3([f64; 3]);

#[derive(Copy, Clone, Debug, Default)]
pub struct Mat4x1([f64; 4]);

#[derive(Copy, Clone, Debug, Default)]
pub struct Mat1x4([f64; 4]);

#[derive(Copy, Clone, Debug, Default)]
pub struct Mat2x2([f64; 4]);

#[derive(Debug, Copy, Clone, Default)]
pub struct Mat3x3([f64; 9]);

#[derive(Debug, Copy, Clone, Default)]
pub struct Mat4x4([f64; 16]);

impl Index<usize> for Mat2x1 {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl IndexMut<usize> for Mat2x1 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl Mul for &Mat2x1 {
    type Output = Mat2x1;
    fn mul(self, rhs: &Mat2x1) -> Self::Output {
        Mat2x1([self.0[0] * rhs.0[0], self.0[1] * rhs.0[1]])
    }
}
impl Mul<f64> for &Mat2x1 {
    type Output = Mat2x1;
    fn mul(self, rhs: f64) -> Self::Output {
        Mat2x1([self.0[0] * rhs, self.0[1] * rhs])
    }
}
impl Div for &Mat2x1 {
    type Output = Mat2x1;
    fn div(self, rhs: &Mat2x1) -> Self::Output {
        Mat2x1([self.0[0] / rhs.0[0], self.0[1] / rhs.0[1]])
    }
}
impl Add for &Mat2x1 {
    type Output = Mat2x1;
    fn add(self, rhs: &Mat2x1) -> Self::Output {
        Mat2x1([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}
impl Sub for &Mat2x1 {
    type Output = Mat2x1;
    fn sub(self, rhs: &Mat2x1) -> Self::Output {
        Mat2x1([self.0[0] - rhs.0[0], self.0[1] - rhs.0[1]])
    }
}
impl Mat2x1 {
    pub fn dot(&self, other: &Mat1x2) -> Mat2x2 {
        Mat2x2([
            self.0[0] * other.0[0],
            self.0[0] * other.0[1],
            self.0[1] * other.0[0],
            self.0[1] * other.0[1],
        ])
    }
    pub fn inner(&self, other: &Mat2x1) -> f64 {
        self.0[0] * other.0[0] + self.0[1] * other.0[1]
    }
    pub fn norm_l2(&self) -> f64 {
        (self.0[0].powi(2) + self.0[1].powi(2)).sqrt()
    }
    pub fn normalize_l2(self) -> Self {
        let m = self.norm_l2();
        Self([self.0[0] / m, self.0[1] / m])
    }
    pub fn t(self) -> Mat1x2 {
        Mat1x2(self.0)
    }
}

impl Index<usize> for Mat1x2 {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl IndexMut<usize> for Mat1x2 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl Mul for &Mat1x2 {
    type Output = Mat1x2;
    fn mul(self, rhs: &Mat1x2) -> Self::Output {
        Mat1x2([self.0[0] * rhs.0[0], self.0[1] * rhs.0[1]])
    }
}
impl Mul<f64> for &Mat1x2 {
    type Output = Mat1x2;
    fn mul(self, rhs: f64) -> Self::Output {
        Mat1x2([self.0[0] * rhs, self.0[1] * rhs])
    }
}
impl Div for &Mat1x2 {
    type Output = Mat1x2;
    fn div(self, rhs: &Mat1x2) -> Self::Output {
        Mat1x2([self.0[0] / rhs.0[0], self.0[1] / rhs.0[1]])
    }
}
impl Div<f64> for &Mat1x2 {
    type Output = Mat1x2;
    fn div(self, rhs: f64) -> Self::Output {
        Mat1x2([self.0[0] / rhs, self.0[1] / rhs])
    }
}
impl Add for &Mat1x2 {
    type Output = Mat1x2;
    fn add(self, rhs: &Mat1x2) -> Self::Output {
        Mat1x2([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}
impl Sub for &Mat1x2 {
    type Output = Mat1x2;
    fn sub(self, rhs: &Mat1x2) -> Self::Output {
        Mat1x2([self.0[0] - rhs.0[0], self.0[1] - rhs.0[1]])
    }
}
impl Mat1x2 {
    pub fn dot(&self, other: &Mat2x1) -> f64 {
        self.0[0] * other.0[0] + self.0[1] * other.0[1]
    }
    pub fn inner(&self, other: &Mat1x2) -> f64 {
        self.0[0] * other.0[0] + self.0[1] * other.0[1]
    }
    pub fn norm_l2(&self) -> f64 {
        (self.0[0].powi(2) + self.0[1].powi(2)).sqrt()
    }
    pub fn normalize_l2(self) -> Self {
        let m = self.norm_l2();
        Self([self.0[0] / m, self.0[1] / m])
    }
    pub fn t(self) -> Mat2x1 {
        Mat2x1(self.0)
    }
}

impl Index<usize> for Mat3x1 {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl IndexMut<usize> for Mat3x1 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl Mul for &Mat3x1 {
    type Output = Mat3x1;
    fn mul(self, rhs: &Mat3x1) -> Self::Output {
        Mat3x1([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
        ])
    }
}
impl Mul<f64> for &Mat3x1 {
    type Output = Mat3x1;
    fn mul(self, rhs: f64) -> Self::Output {
        Mat3x1([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}
impl Div for &Mat3x1 {
    type Output = Mat3x1;
    fn div(self, other: &Mat3x1) -> Mat3x1 {
        Mat3x1([
            self.0[0] / other.0[0],
            self.0[1] / other.0[1],
            self.0[2] / other.0[2],
        ])
    }
}
impl Div<f64> for &Mat3x1 {
    type Output = Mat3x1;
    fn div(self, other: f64) -> Mat3x1 {
        Mat3x1([self.0[0] / other, self.0[1] / other, self.0[2] / other])
    }
}
impl Add for &Mat3x1 {
    type Output = Mat3x1;
    fn add(self, rhs: &Mat3x1) -> Self::Output {
        Mat3x1([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}
impl Sub for &Mat3x1 {
    type Output = Mat3x1;
    fn sub(self, rhs: &Mat3x1) -> Self::Output {
        Mat3x1([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Mat3x1 {
    pub fn new(i: [f64; 3]) -> Self {
        Self(i)
    }
    pub fn dot(&self, other: &Mat1x3) -> Mat3x3 {
        Mat3x3([
            self.0[0] * other.0[0],
            self.0[0] * other.0[1],
            self.0[0] * other.0[2],
            self.0[1] * other.0[0],
            self.0[1] * other.0[1],
            self.0[1] * other.0[2],
            self.0[2] * other.0[0],
            self.0[2] * other.0[1],
            self.0[2] * other.0[2],
        ])
    }
    pub fn inner(&self, other: &Mat3x1) -> f64 {
        self.0[0] * other.0[0] + self.0[1] * other.0[1] + self.0[2] * other.0[2]
    }
    pub fn cross(&self, other: &Self) -> Self {
        Mat3x1([
            ((self.0[1] * other.0[2]) - (self.0[2] * other.0[1])),
            ((self.0[2] * other.0[0]) - (self.0[0] * other.0[2])),
            ((self.0[0] * other.0[1]) - (self.0[1] * other.0[0])),
        ])
    }
    pub fn norm_l2(&self) -> f64 {
        (self.0[0].powi(2) + self.0[1].powi(2) + self.0[2].powi(2)).sqrt()
    }
    pub fn normalize_l2(self) -> Self {
        let m = self.norm_l2();
        Mat3x1([self.0[0] / m, self.0[1] / m, self.0[2] / m])
    }
    pub fn t(self) -> Mat1x3 {
        Mat1x3(self.0)
    }
    pub fn equal(&self, b: &Mat3x1) -> bool {
        for i in 0..3 {
            if !((self.0[i] - b.0[i]).abs() < EPS) {
                return false;
            }
        }
        true
    }
}

impl Index<usize> for Mat1x3 {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl IndexMut<usize> for Mat1x3 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl Mul for &Mat1x3 {
    type Output = Mat1x3;
    fn mul(self, rhs: &Mat1x3) -> Self::Output {
        Mat1x3([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
        ])
    }
}
impl Mul<f64> for &Mat1x3 {
    type Output = Mat1x3;
    fn mul(self, rhs: f64) -> Self::Output {
        Mat1x3([self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs])
    }
}
impl Div for &Mat1x3 {
    type Output = Mat1x3;
    fn div(self, other: &Mat1x3) -> Mat1x3 {
        Mat1x3([
            self.0[0] / other.0[0],
            self.0[1] / other.0[1],
            self.0[2] / other.0[2],
        ])
    }
}
impl Div<f64> for &Mat1x3 {
    type Output = Mat1x3;
    fn div(self, other: f64) -> Mat1x3 {
        Mat1x3([self.0[0] / other, self.0[1] / other, self.0[2] / other])
    }
}
impl Add for &Mat1x3 {
    type Output = Mat1x3;
    fn add(self, rhs: &Mat1x3) -> Self::Output {
        Mat1x3([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}
impl Sub for &Mat1x3 {
    type Output = Mat1x3;
    fn sub(self, rhs: &Mat1x3) -> Self::Output {
        Mat1x3([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Mat1x3 {
    pub fn new(i: [f64; 3]) -> Self {
        Self(i)
    }
    pub fn dot(&self, other: &Mat3x3) -> Mat1x3 {
        Mat1x3([
            self.0[0] * other.0[0] + self.0[1] * other.0[3] + self.0[2] * other.0[6],
            self.0[0] * other.0[1] + self.0[1] * other.0[4] + self.0[2] * other.0[7],
            self.0[0] * other.0[2] + self.0[1] * other.0[5] + self.0[2] * other.0[8],
        ])
    }
    pub fn dot_vec(&self, other: &Mat3x1) -> f64 {
        self.0[0] * other.0[0] + self.0[1] * other.0[1] + self.0[2] * other.0[2]
    }
    pub fn inner(&self, other: &Mat1x3) -> f64 {
        self.0[0] * other.0[0] + self.0[1] * other.0[1] + self.0[2] * other.0[2]
    }
    pub fn cross(&self, other: &Self) -> Self {
        Mat1x3([
            ((self.0[1] * other.0[2]) - (self.0[2] * other.0[1])),
            ((self.0[2] * other.0[0]) - (self.0[0] * other.0[2])),
            ((self.0[0] * other.0[1]) - (self.0[1] * other.0[0])),
        ])
    }
    pub fn norm_l2(&self) -> f64 {
        (self.0[0].powi(2) + self.0[1].powi(2) + self.0[2].powi(2)).sqrt()
    }
    pub fn normalize_l2(&self) -> Self {
        let m = self.norm_l2();
        Mat1x3([self.0[0] / m, self.0[1] / m, self.0[2] / m])
    }
    pub fn t(&self) -> Mat3x1 {
        Mat3x1(self.0)
    }
    pub fn equal(&self, other: &Self) -> bool {
        self.0[0] == other.0[0] && self.0[1] == other.0[1] && self.0[2] == other.0[2]
    }
}

impl Index<usize> for Mat4x1 {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl IndexMut<usize> for Mat4x1 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl Mul for &Mat4x1 {
    type Output = Mat4x1;
    fn mul(self, rhs: &Mat4x1) -> Self::Output {
        Mat4x1([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
            self.0[3] * rhs.0[3],
        ])
    }
}
impl Mul<f64> for &Mat4x1 {
    type Output = Mat4x1;
    fn mul(self, rhs: f64) -> Self::Output {
        Mat4x1([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
        ])
    }
}
impl Div for &Mat4x1 {
    type Output = Mat4x1;
    fn div(self, other: &Mat4x1) -> Mat4x1 {
        Mat4x1([
            self.0[0] / other.0[0],
            self.0[1] / other.0[1],
            self.0[2] / other.0[2],
            self.0[3] / other.0[3],
        ])
    }
}
impl Div<f64> for &Mat4x1 {
    type Output = Mat4x1;
    fn div(self, other: f64) -> Mat4x1 {
        Mat4x1([
            self.0[0] / other,
            self.0[1] / other,
            self.0[2] / other,
            self.0[3] / other,
        ])
    }
}
impl Add for &Mat4x1 {
    type Output = Mat4x1;
    fn add(self, rhs: &Mat4x1) -> Self::Output {
        Mat4x1([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }
}
impl Sub for &Mat4x1 {
    type Output = Mat4x1;
    fn sub(self, rhs: &Mat4x1) -> Self::Output {
        Mat4x1([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
        ])
    }
}

impl Mat4x1 {
    pub fn new(i: [f64; 4]) -> Self {
        Mat4x1(i)
    }
    pub fn shape(&self) -> &'static [usize] {
        &[4, 1]
    }
    pub fn dot_vec(&self, other: &Mat1x4) -> Mat4x4 {
        Mat4x4([
            self.0[0] * other.0[0],
            self.0[0] * other.0[1],
            self.0[0] * other.0[2],
            self.0[0] * other.0[3],
            self.0[1] * other.0[0],
            self.0[1] * other.0[1],
            self.0[1] * other.0[2],
            self.0[1] * other.0[3],
            self.0[2] * other.0[0],
            self.0[2] * other.0[1],
            self.0[2] * other.0[2],
            self.0[2] * other.0[3],
            self.0[3] * other.0[0],
            self.0[3] * other.0[1],
            self.0[3] * other.0[2],
            self.0[3] * other.0[3],
        ])
    }
    pub fn inner(&self, other: &Mat4x1) -> f64 {
        self.0[0] * other.0[0]
            + self.0[1] * other.0[1]
            + self.0[2] * other.0[2]
            + self.0[3] * other.0[3]
    }
    pub fn vec3(&self) -> Mat3x1 {
        Mat3x1([self.0[0], self.0[1], self.0[2]])
    }
    pub fn norm_l2(&self) -> f64 {
        (self.0[0].powi(2) + self.0[1].powi(2) + self.0[2].powi(2) + self.0[3].powi(2)).sqrt()
    }
    pub fn normalize(self) -> Self {
        let m = self.norm_l2();
        Mat4x1([self.0[0] / m, self.0[1] / m, self.0[2] / m, self.0[3] / m])
    }
    pub fn t(self) -> Mat1x4 {
        Mat1x4(self.0)
    }
    pub fn equal(&self, b: &Mat4x1) -> bool {
        for i in 0..4 {
            if !((self.0[i] - b.0[i]).abs() < EPS) {
                return false;
            }
        }
        true
    }
}

impl Mul for &Mat1x4 {
    type Output = Mat1x4;
    fn mul(self, rhs: &Mat1x4) -> Self::Output {
        Mat1x4([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
            self.0[3] * rhs.0[3],
        ])
    }
}
impl Mul<f64> for &Mat1x4 {
    type Output = Mat1x4;
    fn mul(self, rhs: f64) -> Self::Output {
        Mat1x4([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
        ])
    }
}
impl Div for &Mat1x4 {
    type Output = Mat1x4;
    fn div(self, other: &Mat1x4) -> Mat1x4 {
        Mat1x4([
            self.0[0] / other.0[0],
            self.0[1] / other.0[1],
            self.0[2] / other.0[2],
            self.0[3] / other.0[3],
        ])
    }
}
impl Div<f64> for &Mat1x4 {
    type Output = Mat1x4;
    fn div(self, other: f64) -> Mat1x4 {
        Mat1x4([
            self.0[0] / other,
            self.0[1] / other,
            self.0[2] / other,
            self.0[3] / other,
        ])
    }
}
impl Add for &Mat1x4 {
    type Output = Mat1x4;
    fn add(self, rhs: &Mat1x4) -> Self::Output {
        Mat1x4([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }
}
impl Sub for &Mat1x4 {
    type Output = Mat1x4;
    fn sub(self, rhs: &Mat1x4) -> Self::Output {
        Mat1x4([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
        ])
    }
}

impl Mat1x4 {
    pub fn new(i: [f64; 4]) -> Self {
        Self(i)
    }
    pub fn dot_vec(&self, other: &Mat4x1) -> f64 {
        self.0[0] * other.0[0]
            + self.0[1] * other.0[1]
            + self.0[2] * other.0[2]
            + self.0[3] * other.0[3]
    }
    pub fn dot(&self, other: &Mat4x4) -> Mat1x4 {
        Mat1x4([
            self.0[0] * other.0[0]
                + self.0[1] * other.0[4]
                + self.0[2] * other.0[8]
                + self.0[3] * other.0[12],
            self.0[0] * other.0[1]
                + self.0[1] * other.0[5]
                + self.0[2] * other.0[9]
                + self.0[3] * other.0[13],
            self.0[0] * other.0[2]
                + self.0[1] * other.0[6]
                + self.0[2] * other.0[10]
                + self.0[3] * other.0[14],
            self.0[0] * other.0[3]
                + self.0[1] * other.0[7]
                + self.0[2] * other.0[11]
                + self.0[3] * other.0[15],
        ])
    }
    pub fn inner(&self, other: &Mat1x4) -> f64 {
        self.0[0] * other.0[0]
            + self.0[1] * other.0[1]
            + self.0[2] * other.0[2]
            + self.0[3] * other.0[3]
    }
    pub fn vec3(&self) -> Mat1x3 {
        Mat1x3([self.0[0], self.0[1], self.0[2]])
    }
    pub fn norm_l2(&self) -> f64 {
        (self.0[0].powi(2) + self.0[1].powi(2) + self.0[2].powi(2) + self.0[3].powi(2)).sqrt()
    }
    pub fn normalize(self) -> Self {
        let m = self.norm_l2();
        Mat1x4([self.0[0] / m, self.0[1] / m, self.0[2] / m, self.0[3] / m])
    }
    pub fn t(self) -> Mat4x1 {
        Mat4x1(self.0)
    }
    pub fn equal(&self, b: &Mat1x4) -> bool {
        for i in 0..4 {
            if !((self.0[i] - b.0[i]).abs() < EPS) {
                return false;
            }
        }
        true
    }
}

impl Index<[usize; 2]> for Mat3x3 {
    type Output = f64;
    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        &self.0[idx[0] * 3 + idx[1]]
    }
}

impl IndexMut<[usize; 2]> for Mat3x3 {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Self::Output {
        &mut self.0[idx[0] * 3 + idx[1]]
    }
}

impl Mul for &Mat3x3 {
    type Output = Mat3x3;
    fn mul(self, rhs: &Mat3x3) -> Self::Output {
        Mat3x3::new_r([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
            self.0[3] * rhs.0[3],
            self.0[4] * rhs.0[4],
            self.0[5] * rhs.0[5],
            self.0[6] * rhs.0[6],
            self.0[7] * rhs.0[7],
            self.0[8] * rhs.0[8],
        ])
    }
}
impl Mul<f64> for &Mat3x3 {
    type Output = Mat3x3;
    fn mul(self, rhs: f64) -> Self::Output {
        Mat3x3::new_r([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
            self.0[4] * rhs,
            self.0[5] * rhs,
            self.0[6] * rhs,
            self.0[7] * rhs,
            self.0[8] * rhs,
        ])
    }
}
impl Div for &Mat3x3 {
    type Output = Mat3x3;
    fn div(self, rhs: &Mat3x3) -> Self::Output {
        Mat3x3::new_r([
            self.0[0] / rhs.0[0],
            self.0[1] / rhs.0[1],
            self.0[2] / rhs.0[2],
            self.0[3] / rhs.0[3],
            self.0[4] / rhs.0[4],
            self.0[5] / rhs.0[5],
            self.0[6] / rhs.0[6],
            self.0[7] / rhs.0[7],
            self.0[8] / rhs.0[8],
        ])
    }
}
impl Div<f64> for &Mat3x3 {
    type Output = Mat3x3;
    fn div(self, rhs: f64) -> Self::Output {
        Mat3x3::new_r([
            self.0[0] / rhs,
            self.0[1] / rhs,
            self.0[2] / rhs,
            self.0[3] / rhs,
            self.0[4] / rhs,
            self.0[5] / rhs,
            self.0[6] / rhs,
            self.0[7] / rhs,
            self.0[8] / rhs,
        ])
    }
}
impl Add for &Mat3x3 {
    type Output = Mat3x3;
    fn add(self, rhs: &Mat3x3) -> Self::Output {
        Mat3x3::new_r([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
            self.0[4] + rhs.0[4],
            self.0[5] + rhs.0[5],
            self.0[6] + rhs.0[6],
            self.0[7] + rhs.0[7],
            self.0[8] + rhs.0[8],
        ])
    }
}
impl Sub for &Mat3x3 {
    type Output = Mat3x3;
    fn sub(self, rhs: &Mat3x3) -> Self::Output {
        Mat3x3::new_r([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
            self.0[4] - rhs.0[4],
            self.0[5] - rhs.0[5],
            self.0[6] - rhs.0[6],
            self.0[7] - rhs.0[7],
            self.0[8] - rhs.0[8],
        ])
    }
}
impl Mat3x3 {
    ///expects row major input
    pub fn new_r(arr: [f64; 9]) -> Mat3x3 {
        Mat3x3(arr)
    }
    ///expects column major input
    pub fn new_c(arr: [f64; 9]) -> Mat3x3 {
        Mat3x3([
            arr[0], arr[3], arr[6], arr[1], arr[4], arr[7], arr[2], arr[5], arr[8],
        ])
    }
    pub fn eye() -> Mat3x3 {
        Mat3x3([1., 0., 0., 0., 1., 0., 0., 0., 1.])
    }
    pub fn trace(&self) -> f64 {
        self.0[0] + self.0[4] + self.0[8]
    }
    pub fn dot_vec(&self, other: &Mat3x1) -> Mat3x1 {
        let mut m = Mat3x1::default();
        for y in 0..3 {
            m.0[y] = self.0[y * 3 + 0] * other.0[0]
                + self.0[y * 3 + 1] * other.0[1]
                + self.0[y * 3 + 2] * other.0[2];
        }
        m
    }
    pub fn dot(&self, other: &Mat3x3) -> Mat3x3 {
        let a = arr2(&[
            [self.0[0], self.0[1], self.0[2]],
            [self.0[3], self.0[4], self.0[5]],
            [self.0[6], self.0[7], self.0[8]],
        ]);

        let b = arr2(&[
            [other.0[0], other.0[1], other.0[2]],
            [other.0[3], other.0[4], other.0[5]],
            [other.0[6], other.0[7], other.0[8]],
        ]);

        let c = a.dot(&b);
        Mat3x3::new_r([
            c[[0, 0]],
            c[[0, 1]],
            c[[0, 2]],
            c[[1, 0]],
            c[[1, 1]],
            c[[1, 2]],
            c[[2, 0]],
            c[[2, 1]],
            c[[2, 2]],
        ])
    }
    pub fn t(self) -> Mat3x3 {
        let mut copy = Mat3x3::default();
        for i in 0..3 {
            for j in 0..3 {
                copy.0[i * 3 + j] = self.0[j * 3 + i];
            }
        }
        copy
    }
    pub fn inverse(&self) -> Result<Mat3x3, &'static str> {
        use ndarray_linalg::solve::Inverse;

        let a = arr2(&[
            [self.0[0], self.0[1], self.0[2]],
            [self.0[3], self.0[4], self.0[5]],
            [self.0[6], self.0[7], self.0[8]],
        ]);

        match a.inv() {
            Ok(c) => Ok(Mat3x3::new_r([
                c[[0, 0]],
                c[[0, 1]],
                c[[0, 2]],
                c[[1, 0]],
                c[[1, 1]],
                c[[1, 2]],
                c[[2, 0]],
                c[[2, 1]],
                c[[2, 2]],
            ])),
            Err(_) => Err("inverse error"),
        }
    }
    pub fn equal(&self, b: &Mat3x3) -> bool {
        for i in 0..9 {
            if !((self.0[i] - b.0[i]).abs() < EPS) {
                return false;
            }
        }
        true
    }
}

impl Index<[usize; 2]> for Mat4x4 {
    type Output = f64;
    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        &self.0[idx[0] * 3 + idx[1]]
    }
}

impl IndexMut<[usize; 2]> for Mat4x4 {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Self::Output {
        &mut self.0[idx[0] * 3 + idx[1]]
    }
}

impl Mul for &Mat4x4 {
    type Output = Mat4x4;
    fn mul(self, rhs: &Mat4x4) -> Self::Output {
        Mat4x4::new_r([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
            self.0[3] * rhs.0[3],
            self.0[4] * rhs.0[4],
            self.0[5] * rhs.0[5],
            self.0[6] * rhs.0[6],
            self.0[7] * rhs.0[7],
            self.0[8] * rhs.0[8],
            self.0[9] * rhs.0[9],
            self.0[10] * rhs.0[10],
            self.0[11] * rhs.0[11],
            self.0[12] * rhs.0[12],
            self.0[13] * rhs.0[13],
            self.0[14] * rhs.0[14],
            self.0[15] * rhs.0[15],
        ])
    }
}
impl Mul<f64> for &Mat4x4 {
    type Output = Mat4x4;
    fn mul(self, rhs: f64) -> Self::Output {
        Mat4x4::new_r([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
            self.0[4] * rhs,
            self.0[5] * rhs,
            self.0[6] * rhs,
            self.0[7] * rhs,
            self.0[8] * rhs,
            self.0[9] * rhs,
            self.0[10] * rhs,
            self.0[11] * rhs,
            self.0[12] * rhs,
            self.0[13] * rhs,
            self.0[14] * rhs,
            self.0[15] * rhs,
        ])
    }
}
impl Div for &Mat4x4 {
    type Output = Mat4x4;
    fn div(self, rhs: &Mat4x4) -> Self::Output {
        Mat4x4::new_r([
            self.0[0] / rhs.0[0],
            self.0[1] / rhs.0[1],
            self.0[2] / rhs.0[2],
            self.0[3] / rhs.0[3],
            self.0[4] / rhs.0[4],
            self.0[5] / rhs.0[5],
            self.0[6] / rhs.0[6],
            self.0[7] / rhs.0[7],
            self.0[8] / rhs.0[8],
            self.0[9] / rhs.0[9],
            self.0[10] / rhs.0[10],
            self.0[11] / rhs.0[11],
            self.0[12] / rhs.0[12],
            self.0[13] / rhs.0[13],
            self.0[14] / rhs.0[14],
            self.0[15] / rhs.0[15],
        ])
    }
}
impl Div<f64> for &Mat4x4 {
    type Output = Mat4x4;
    fn div(self, rhs: f64) -> Self::Output {
        Mat4x4::new_r([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
            self.0[4] * rhs,
            self.0[5] * rhs,
            self.0[6] * rhs,
            self.0[7] * rhs,
            self.0[8] * rhs,
            self.0[9] * rhs,
            self.0[10] * rhs,
            self.0[11] * rhs,
            self.0[12] * rhs,
            self.0[13] * rhs,
            self.0[14] * rhs,
            self.0[15] * rhs,
        ])
    }
}
impl Add for &Mat4x4 {
    type Output = Mat4x4;
    fn add(self, rhs: &Mat4x4) -> Self::Output {
        Mat4x4::new_r([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
            self.0[4] + rhs.0[4],
            self.0[5] + rhs.0[5],
            self.0[6] + rhs.0[6],
            self.0[7] + rhs.0[7],
            self.0[8] + rhs.0[8],
            self.0[9] + rhs.0[9],
            self.0[10] + rhs.0[10],
            self.0[11] + rhs.0[11],
            self.0[12] + rhs.0[12],
            self.0[13] + rhs.0[13],
            self.0[14] + rhs.0[14],
            self.0[15] + rhs.0[15],
        ])
    }
}
impl Sub for &Mat4x4 {
    type Output = Mat4x4;
    fn sub(self, rhs: &Mat4x4) -> Self::Output {
        Mat4x4::new_r([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
            self.0[4] - rhs.0[4],
            self.0[5] - rhs.0[5],
            self.0[6] - rhs.0[6],
            self.0[7] - rhs.0[7],
            self.0[8] - rhs.0[8],
            self.0[9] - rhs.0[9],
            self.0[10] - rhs.0[10],
            self.0[11] - rhs.0[11],
            self.0[12] - rhs.0[12],
            self.0[13] - rhs.0[13],
            self.0[14] - rhs.0[14],
            self.0[15] - rhs.0[15],
        ])
    }
}

impl Mat4x4 {
    ///expects row major input
    pub fn new_r(arr: [f64; 16]) -> Mat4x4 {
        Mat4x4(arr)
    }
    ///expects column major input
    pub fn new_c(arr: [f64; 16]) -> Mat4x4 {
        Mat4x4([
            arr[0], arr[4], arr[8], arr[12], arr[1], arr[5], arr[9], arr[13], arr[2], arr[6],
            arr[10], arr[14], arr[3], arr[7], arr[11], arr[15],
        ])
    }
    pub fn eye() -> Mat4x4 {
        Mat4x4([
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ])
    }
    pub fn trace(&self) -> f64 {
        self.0[0] + self.0[4] + self.0[8] + self.0[12]
    }
    pub fn dot_vec(&self, other: &Mat4x1) -> Mat4x1 {
        let mut m = Mat4x1::default();
        for y in 0..4 {
            m.0[y] = self.0[y * 4 + 0] * other.0[0]
                + self.0[y * 4 + 1] * other.0[1]
                + self.0[y * 4 + 2] * other.0[2]
                + self.0[y * 4 + 3] * other.0[3];
        }
        m
    }
    pub fn dot(&self, other: &Mat4x4) -> Mat4x4 {
        let a = arr2(&[
            [self.0[0], self.0[1], self.0[2], self.0[3]],
            [self.0[4], self.0[5], self.0[6], self.0[7]],
            [self.0[8], self.0[9], self.0[10], self.0[11]],
            [self.0[12], self.0[13], self.0[14], self.0[15]],
        ]);

        let b = arr2(&[
            [other.0[0], other.0[1], other.0[2], other.0[3]],
            [other.0[4], other.0[5], other.0[6], other.0[7]],
            [other.0[8], other.0[9], other.0[10], other.0[11]],
            [other.0[12], other.0[13], other.0[14], other.0[15]],
        ]);

        let c = a.dot(&b);

        Mat4x4::new_r([
            c[[0, 0]],
            c[[0, 1]],
            c[[0, 2]],
            c[[0, 3]],
            c[[1, 0]],
            c[[1, 1]],
            c[[1, 2]],
            c[[1, 3]],
            c[[2, 0]],
            c[[2, 1]],
            c[[2, 2]],
            c[[2, 3]],
            c[[3, 0]],
            c[[3, 1]],
            c[[3, 2]],
            c[[3, 3]],
        ])
    }
    pub fn t(self) -> Mat4x4 {
        let mut copy = Mat4x4::default();
        for i in 0..4 {
            for j in 0..4 {
                copy.0[i * 4 + j] = self.0[j * 4 + i];
            }
        }
        copy
    }
    pub fn inverse(&self) -> Result<Mat4x4, &'static str> {
        use ndarray_linalg::solve::Inverse;

        let a = arr2(&[
            [self.0[0], self.0[1], self.0[2], self.0[3]],
            [self.0[4], self.0[5], self.0[6], self.0[7]],
            [self.0[8], self.0[9], self.0[10], self.0[11]],
            [self.0[12], self.0[13], self.0[14], self.0[15]],
        ]);

        match a.inv() {
            Ok(c) => Ok(Mat4x4::new_r([
                c[[0, 0]],
                c[[0, 1]],
                c[[0, 2]],
                c[[0, 3]],
                c[[1, 0]],
                c[[1, 1]],
                c[[1, 2]],
                c[[1, 3]],
                c[[2, 0]],
                c[[2, 1]],
                c[[2, 2]],
                c[[2, 3]],
                c[[3, 0]],
                c[[3, 1]],
                c[[3, 2]],
                c[[3, 3]],
            ])),
            Err(_) => Err("inverse error"),
        }
    }
    ///extract upper left 3x3 matrix
    pub fn sub_rot(&self) -> Mat3x3 {
        Mat3x3::new_r([
            self.0[0], self.0[1], self.0[2], self.0[4], self.0[5], self.0[6], self.0[8], self.0[9],
            self.0[10],
        ])
    }
    ///extract 3x1 translation
    pub fn sub_xlate(&self) -> Mat3x1 {
        Mat3x1::new([self.0[2], self.0[5], self.0[8]])
    }
    pub fn equal(&self, b: &Mat4x4) -> bool {
        for i in 0..16 {
            if !((self.0[i] - b.0[i]).abs() < EPS) {
                return false;
            }
        }
        true
    }
}

impl From<Mat3x3> for Matrix {
    fn from(i: Mat3x3) -> Matrix {
        Matrix(arr2(&[
            [i.0[0], i.0[1], i.0[2]],
            [i.0[3], i.0[4], i.0[5]],
            [i.0[6], i.0[7], i.0[8]],
        ]))
    }
}

impl Into<Result<Mat3x3, &'static str>> for Matrix {
    fn into(self) -> Result<Mat3x3, &'static str> {
        match self.shape() {
            &[3, 3] => Ok(Mat3x3::new_r([
                self[[0, 0]],
                self[[0, 1]],
                self[[0, 2]],
                self[[1, 0]],
                self[[1, 1]],
                self[[1, 2]],
                self[[2, 0]],
                self[[2, 1]],
                self[[2, 2]],
            ])),
            _ => Err("dimension mismatch"),
        }
    }
}

impl From<Mat4x4> for Matrix {
    fn from(i: Mat4x4) -> Matrix {
        Matrix::from(arr2(&[
            [i.0[0], i.0[1], i.0[2], i.0[3]],
            [i.0[4], i.0[5], i.0[6], i.0[7]],
            [i.0[8], i.0[9], i.0[10], i.0[11]],
            [i.0[12], i.0[13], i.0[14], i.0[15]],
        ]))
    }
}

#[derive(Debug, Clone)]
pub struct Matrix1D(Array<f64, Ix1>);
#[derive(Debug, Clone)]
pub struct Matrix1DView<'a>(ArrayView<'a, f64, Ix1>);
#[derive(Debug, Clone)]
pub struct Matrix(Array<f64, Ix2>);
#[derive(Debug, Clone)]
pub struct MatrixView<'a>(ArrayView<'a, f64, Ix2>);

impl Default for Matrix1D {
    fn default() -> Self {
        Matrix1D::from(arr1(&[0., 0., 0.]))
    }
}
impl Default for Matrix {
    fn default() -> Self {
        Matrix::from(arr2(&[[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]]))
    }
}

#[derive(Default)]
pub struct Arrayf32_16(pub [f32; 16]);

#[derive(Default)]
pub struct Arrayf32_9(pub [f32; 9]);

#[derive(Default)]
pub struct Arrayf32_4(pub [f32; 4]);

#[derive(Default)]
pub struct Arrayf32_3(pub [f32; 3]);

impl From<Array<f64, Ix2>> for Matrix {
    fn from(i: Array<f64, Ix2>) -> Matrix {
        Matrix(i)
    }
}

impl<'a> From<ArrayView<'a, f64, Ix2>> for MatrixView<'a> {
    fn from(i: ArrayView<'a, f64, Ix2>) -> MatrixView<'a> {
        MatrixView(i)
    }
}

impl From<Array<f64, Ix1>> for Matrix1D {
    fn from(i: Array<f64, Ix1>) -> Matrix1D {
        Matrix1D(i)
    }
}

impl<'a> From<ArrayView<'a, f64, Ix1>> for Matrix1DView<'a> {
    fn from(i: ArrayView<'a, f64, Ix1>) -> Matrix1DView<'a> {
        Matrix1DView(i)
    }
}

impl From<Matrix1D> for Mat3x1 {
    ///convert to column major ordering flattened array
    fn from(m: Matrix1D) -> Self {
        Self::new([m[0], m[1], m[2]])
    }
}

impl From<Matrix1D> for Mat4x1 {
    ///convert to column major ordering flattened array
    fn from(m: Matrix1D) -> Self {
        Self::new([m[0], m[1], m[2], m[3]])
    }
}

impl From<Matrix1D> for Mat1x3 {
    ///convert to column major ordering flattened array
    fn from(m: Matrix1D) -> Self {
        Self::new([m[0], m[1], m[2]])
    }
}

impl From<Matrix1D> for Mat1x4 {
    ///convert to column major ordering flattened array
    fn from(m: Matrix1D) -> Self {
        Self::new([m[0], m[1], m[2], m[3]])
    }
}

impl From<Matrix> for Arrayf32_16 {
    ///convert to column major ordering flattened array
    fn from(m: Matrix) -> Self {
        let mut arr = [0f32; 16];
        for (idx, i) in m.0.t().iter().take(16).enumerate() {
            arr[idx] = *i as _;
        }
        Self(arr)
    }
}

impl From<Matrix> for Arrayf32_9 {
    ///convert to column major ordering flattened array
    fn from(m: Matrix) -> Self {
        let mut arr = [0f32; 9];
        for (idx, i) in m.0.t().iter().take(9).enumerate() {
            arr[idx] = *i as _;
        }
        Self(arr)
    }
}

impl From<Matrix> for Arrayf32_4 {
    ///convert to column major ordering flattened array
    fn from(m: Matrix) -> Self {
        let mut arr = [0f32; 4];
        for (idx, i) in m.0.t().iter().take(4).enumerate() {
            arr[idx] = *i as _;
        }
        Self(arr)
    }
}

impl From<Matrix> for Arrayf32_3 {
    ///convert to column major ordering flattened array
    fn from(m: Matrix) -> Self {
        let mut arr = [0f32; 3];
        for (idx, i) in m.0.t().iter().take(3).enumerate() {
            arr[idx] = *i as _;
        }
        Self(arr)
    }
}

impl<'a> MatrixView<'a> {
    pub fn cross_vec(&self, b: &'a MatrixView) -> Matrix {
        assert!(self.0.shape().len() == 2);
        assert!(b.shape().len() == 2);
        assert!(self.0.shape()[0] >= 3);
        assert!(b.shape()[0] >= 3);
        assert!(self.0.shape()[1] == 1);
        assert!(b.shape()[1] == 1);
        Matrix::from(array![
            [self.0[[1, 0]] * b[[2, 0]] - b[[1, 0]] * self.0[[2, 0]]],
            [-self.0[[0, 0]] * b[[2, 0]] + b[[0, 0]] * self.0[[2, 0]]],
            [self.0[[0, 0]] * b[[1, 0]] - b[[0, 0]] * self.0[[1, 0]]],
            [0.]
        ])
    }
    pub fn cross_vec_1d(&self, b: &'a Matrix1DView) -> Matrix1D {
        assert!(self.0.shape().len() == 1);
        assert!(b.shape().len() == 1);
        assert!(self.0.shape()[0] == 3);
        assert!(b.shape()[0] == 3);
        Matrix1D::from(arr1(&[
            self.0[[1, 0]] * b[2] - b[1] * self.0[[2, 0]],
            -self.0[[0, 0]] * b[2] + b[0] * self.0[[2, 0]],
            self.0[[0, 0]] * b[1] - b[0] * self.0[[1, 0]],
            0.,
        ]))
    }
    pub fn norm_l2(&self) -> f64 {
        (self.0.to_owned() * self.0.to_owned()).sum().sqrt()
    }

    pub fn mag_vec3_l2(&self) -> f64 {
        assert!(self.shape()[0] >= 3);
        let s = self.0.slice(s![0..3, ..]);
        s.t().dot(&s).sum().sqrt()
    }
    pub fn normalize_l2(&self) -> Matrix {
        let m = self.norm_l2();
        let factor = 1.0 / m;
        self * factor
    }
    pub fn t(&self) -> MatrixView {
        MatrixView(self.0.t())
    }
}

impl Matrix {
    pub fn cross_vec(&self, b: &Matrix) -> Matrix {
        assert!(self.0.shape().len() == 2);
        assert!(b.shape().len() == 2);
        assert!(self.0.shape()[0] >= 3);
        assert!(b.shape()[0] >= 3);
        assert!(self.0.shape()[1] == 1);
        assert!(b.shape()[1] == 1);
        Self(array![
            [self.0[[1, 0]] * b[[2, 0]] - b[[1, 0]] * self.0[[2, 0]]],
            [-self.0[[0, 0]] * b[[2, 0]] + b[[0, 0]] * self.0[[2, 0]]],
            [self.0[[0, 0]] * b[[1, 0]] - b[[0, 0]] * self.0[[1, 0]]],
            [0.]
        ])
    }
    pub fn cross_vec_1d(&self, b: &Matrix1D) -> Matrix1D {
        assert!(self.0.shape().len() == 1);
        assert!(b.shape().len() == 1);
        assert!(self.0.shape()[0] == 3);
        assert!(b.shape()[0] == 3);
        Matrix1D(arr1(&[
            self.0[[1, 0]] * b[2] - b[1] * self.0[[2, 0]],
            -self.0[[0, 0]] * b[2] + b[0] * self.0[[2, 0]],
            self.0[[0, 0]] * b[1] - b[0] * self.0[[1, 0]],
            0.,
        ]))
    }
    pub fn norm_l2(&self) -> f64 {
        (&self.0 * &self.0).sum().sqrt()
    }

    pub fn norm_vec3_l2(&self) -> f64 {
        assert!(self.shape()[0] >= 3);
        let s = self.0.slice(s![0..3, ..]);
        s.t().dot(&s).sum().sqrt()
    }
    pub fn normalize_l2(&self) -> Matrix {
        let m = self.norm_l2();
        let factor = 1.0 / m;
        let b = self.to_owned();
        b * factor
    }
    pub fn dot(&self, other: &Self) -> Self {
        Matrix(self.0.dot(&other.0))
    }
    pub fn t(&self) -> MatrixView {
        MatrixView(self.0.t())
    }
    pub fn shape(&self) -> &[usize] {
        self.0.shape()
    }
    pub fn view(&self) -> MatrixView {
        MatrixView(self.0.view())
    }
}

impl<'a> MatrixView<'a> {
    pub fn shape(&self) -> &[usize] {
        self.0.shape()
    }
}

impl<'a> Matrix1DView<'a> {
    pub fn norm_l2(&self) -> f64 {
        (self.0.to_owned() * self.0.to_owned()).sum().sqrt()
    }
    pub fn normalize_l2(&self) -> Matrix1D {
        let m = self.norm_l2();
        let factor = 1.0 / m;
        self * factor
    }
    pub fn dot(&self, other: &'a Matrix1DView) -> f64 {
        (&self.0 * &other.0).sum()
    }
    pub fn t(&'a self) -> Matrix1DView<'a> {
        Matrix1DView(self.0.t())
    }
    pub fn shape(&self) -> &[usize] {
        self.0.shape()
    }
}

impl Matrix1D {
    pub fn cross_vec_1d(&self, b: &Matrix1D) -> Matrix1D {
        assert!(self.0.shape().len() == 1);
        assert!(b.shape().len() == 1);
        assert!(self.0.shape()[0] == 3);
        assert!(b.shape()[0] == 3);
        Matrix1D(arr1(&[
            self.0[1] * b[2] - b[1] * self.0[2],
            -self.0[0] * b[2] + b[0] * self.0[2],
            self.0[0] * b[1] - b[0] * self.0[1],
        ]))
    }
    pub fn norm_l2(&self) -> f64 {
        (&self.0 * &self.0).sum().sqrt()
    }
    pub fn normalize_l2(&self) -> Matrix1D {
        let m = self.norm_l2();
        let factor = 1.0 / m;
        Self(&self.0 * factor)
    }
    pub fn inner(&self, other: &Self) -> f64 {
        (&self.0 * &other.0).sum()
    }
    pub fn t(&self) -> Matrix1DView {
        Matrix1DView(self.0.t())
    }
    pub fn shape(&self) -> &[usize] {
        self.0.shape()
    }
    pub fn view(&self) -> Matrix1DView {
        Matrix1DView(self.0.view())
    }
    pub fn sum(&self) -> f64 {
        self.0.sum()
    }
}

impl Mul for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Self::Output {
        Matrix(&self.0 * &rhs.0)
    }
}
impl Mul<f64> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix(&self.0 * rhs)
    }
}
impl Div for &Matrix {
    type Output = Matrix;
    fn div(self, rhs: &Matrix) -> Self::Output {
        Matrix(&self.0 / &rhs.0)
    }
}
impl Add for &Matrix {
    type Output = Matrix;
    fn add(self, rhs: &Matrix) -> Self::Output {
        Matrix(&self.0 + &rhs.0)
    }
}
impl Sub for &Matrix {
    type Output = Matrix;
    fn sub(self, rhs: &Matrix) -> Self::Output {
        Matrix(&self.0 - &rhs.0)
    }
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        Self(&self.0 * &rhs.0)
    }
}
impl Mul<f64> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(&self.0 * rhs)
    }
}
impl Div for Matrix {
    type Output = Matrix;
    fn div(self, rhs: Matrix) -> Self::Output {
        Self(&self.0 / &rhs.0)
    }
}
impl Add for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Self::Output {
        Self(&self.0 + &rhs.0)
    }
}
impl Sub for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Self::Output {
        Matrix(&self.0 - &rhs.0)
    }
}

impl<'a> Mul for MatrixView<'a> {
    type Output = Matrix;
    fn mul(self, rhs: MatrixView<'a>) -> Self::Output {
        Matrix(&self.0 * &rhs.0)
    }
}
impl<'a> Mul<f64> for MatrixView<'a> {
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix(&self.0 * rhs)
    }
}
impl<'a> Div for MatrixView<'a> {
    type Output = Matrix;
    fn div(self, rhs: MatrixView<'a>) -> Self::Output {
        Matrix(&self.0 / &rhs.0)
    }
}
impl<'a> Add for MatrixView<'a> {
    type Output = Matrix;
    fn add(self, rhs: MatrixView<'a>) -> Self::Output {
        Matrix(&self.0 + &rhs.0)
    }
}
impl<'a> Sub for MatrixView<'a> {
    type Output = Matrix;
    fn sub(self, rhs: MatrixView<'a>) -> Self::Output {
        Matrix(&self.0 - &rhs.0)
    }
}

impl<'a> Mul for &MatrixView<'a> {
    type Output = Matrix;
    fn mul(self, rhs: &MatrixView<'a>) -> Self::Output {
        Matrix(&self.0 * &rhs.0)
    }
}
impl<'a> Mul<f64> for &MatrixView<'a> {
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix(&self.0 * rhs)
    }
}
impl<'a> Div for &MatrixView<'a> {
    type Output = Matrix;
    fn div(self, rhs: &MatrixView<'a>) -> Self::Output {
        Matrix(&self.0 / &rhs.0)
    }
}
impl<'a> Add for &MatrixView<'a> {
    type Output = Matrix;
    fn add(self, rhs: &MatrixView<'a>) -> Self::Output {
        Matrix(&self.0 + &rhs.0)
    }
}
impl<'a> Sub for &MatrixView<'a> {
    type Output = Matrix;
    fn sub(self, rhs: &MatrixView<'a>) -> Self::Output {
        Matrix(&self.0 - &rhs.0)
    }
}

impl Mul for &Matrix1D {
    type Output = Matrix1D;
    fn mul(self, rhs: &Matrix1D) -> Self::Output {
        Matrix1D(&self.0 * &rhs.0)
    }
}
impl Mul<f64> for &Matrix1D {
    type Output = Matrix1D;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix1D(&self.0 * rhs)
    }
}
impl Div for &Matrix1D {
    type Output = Matrix1D;
    fn div(self, rhs: &Matrix1D) -> Self::Output {
        Matrix1D(&self.0 / &rhs.0)
    }
}
impl Add for &Matrix1D {
    type Output = Matrix1D;
    fn add(self, rhs: &Matrix1D) -> Self::Output {
        Matrix1D(&self.0 + &rhs.0)
    }
}
impl Sub for &Matrix1D {
    type Output = Matrix1D;
    fn sub(self, rhs: &Matrix1D) -> Self::Output {
        Matrix1D(&self.0 - &rhs.0)
    }
}

impl Mul for Matrix1D {
    type Output = Matrix1D;
    fn mul(self, rhs: Matrix1D) -> Self::Output {
        Matrix1D(self.0 * rhs.0)
    }
}
impl Mul<f64> for Matrix1D {
    type Output = Matrix1D;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix1D(self.0 * rhs)
    }
}
impl Mul<Matrix1D> for f64 {
    type Output = Matrix1D;
    fn mul(self, rhs: Matrix1D) -> Self::Output {
        Matrix1D(self * rhs.0)
    }
}

impl Div for Matrix1D {
    type Output = Matrix1D;
    fn div(self, rhs: Matrix1D) -> Self::Output {
        Matrix1D(self.0 / rhs.0)
    }
}
impl Add for Matrix1D {
    type Output = Matrix1D;
    fn add(self, rhs: Matrix1D) -> Self::Output {
        Matrix1D(self.0 + rhs.0)
    }
}
impl Sub for Matrix1D {
    type Output = Matrix1D;
    fn sub(self, rhs: Matrix1D) -> Self::Output {
        Matrix1D(self.0 - rhs.0)
    }
}

impl<'a> Mul for &Matrix1DView<'a> {
    type Output = Matrix1D;
    fn mul(self, rhs: &Matrix1DView<'a>) -> Self::Output {
        Matrix1D(&self.0 * &rhs.0)
    }
}
impl<'a> Mul<f64> for &Matrix1DView<'a> {
    type Output = Matrix1D;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix1D(&self.0 * rhs)
    }
}
impl<'a> Div for &Matrix1DView<'a> {
    type Output = Matrix1D;
    fn div(self, rhs: &Matrix1DView<'a>) -> Self::Output {
        Matrix1D(&self.0 / &rhs.0)
    }
}
impl<'a> Add for &Matrix1DView<'a> {
    type Output = Matrix1D;
    fn add(self, rhs: &Matrix1DView<'a>) -> Self::Output {
        Matrix1D(&self.0 + &rhs.0)
    }
}
impl<'a> Sub for &Matrix1DView<'a> {
    type Output = Matrix1D;
    fn sub(self, rhs: &Matrix1DView<'a>) -> Self::Output {
        Matrix1D(&self.0 - &rhs.0)
    }
}

impl<'a> Mul for Matrix1DView<'a> {
    type Output = Matrix1D;
    fn mul(self, rhs: Matrix1DView<'a>) -> Self::Output {
        Matrix1D(&self.0 * &rhs.0)
    }
}
impl<'a> Mul<f64> for Matrix1DView<'a> {
    type Output = Matrix1D;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix1D(&self.0 * rhs)
    }
}
impl<'a> Div for Matrix1DView<'a> {
    type Output = Matrix1D;
    fn div(self, rhs: Matrix1DView<'a>) -> Self::Output {
        Matrix1D(&self.0 / &rhs.0)
    }
}
impl<'a> Add for Matrix1DView<'a> {
    type Output = Matrix1D;
    fn add(self, rhs: Matrix1DView<'a>) -> Self::Output {
        Matrix1D(&self.0 + &rhs.0)
    }
}
impl<'a> Sub for Matrix1DView<'a> {
    type Output = Matrix1D;
    fn sub(self, rhs: Matrix1DView<'a>) -> Self::Output {
        Matrix1D(&self.0 - &rhs.0)
    }
}

impl Index<usize> for Matrix1D {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl IndexMut<usize> for Matrix1D {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl<'a> Index<[usize; 2]> for MatrixView<'a> {
    type Output = f64;
    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        &self.0[idx]
    }
}

impl Index<[usize; 2]> for Matrix {
    type Output = f64;
    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        &self.0[idx]
    }
}

impl IndexMut<[usize; 2]> for Matrix {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl<'a> Index<usize> for Matrix1DView<'a> {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

#[test]
fn test() {
    // {
    //     let a: Matrix1D = arr1(&[1., 2., 3.]);
    //     let b: Matrix = arr2(&[[4., 5., 6.], [7., 8., 9.], [10., 11., 12.]]);
    //     let _c = b.dot(&a);
    // }
    // {
    //     let a = Array::random((4, 1), Uniform::new(0., 10.));
    //     let b = Array::random((4, 1), Uniform::new(0., 10.));
    //     let c = cross_vec(&a.view(), &b.view());
    //     assert_eq!(c.shape(), &[4, 1]);
    // }
    {
        let a = Array::random((4, 1), Uniform::new(0., 10.));
        let b = Array::random((4, 1), Uniform::new(0., 10.));
        let c = &a + &b;
        for i in 0..4 {
            relative_eq!(a[[i, 0]] + b[[i, 0]], c[[i, 0]], epsilon = f64::EPSILON);
        }
    }
    {
        let a = Array::random((4, 1), Uniform::new(0., 10.));
        let b = Array::random((4, 1), Uniform::new(0., 10.));
        let c = &a * &b;

        assert_eq!(c.shape(), &[4, 1]);
        for i in 0..4 {
            relative_eq!(a[[i, 0]] * b[[i, 0]], c[[i, 0]], epsilon = f64::EPSILON);
        }
    }
    {
        let a = Array::random((4, 1), Uniform::new(0., 10.));
        let b = Array::random((4, 1), Uniform::new(0., 10.));
        let c = a.dot(&b.t());
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(c[[i, j]], a[[i, 0]] * b[[j, 0]]);
            }
        }
    }
    // {
    //     let a = Array::random((3, 1), Uniform::new(0., 10.));
    //     let m = mag_vec3_l2(&a.view());
    //     relative_eq!(
    //         m,
    //         (0..3).map(|i| a[[i, 0]] * a[[i, 0]]).sum::<f64>().sqrt(),
    //         epsilon = f64::EPSILON
    //     );
    // }
    // {
    //     let aa = array![[4.0], [5.0], [2.0], [3.0]];
    //     let m = normalize_vec_l2(&aa.view());
    //     assert!(m.shape() == &[4, 1]);
    //     let factor = 1.0f64 / (4.0f64 * 4.0 + 5.0 * 5.0 + 2.0 * 2.0 + 3.0 * 3.0).sqrt();
    //     let expect = array![[4.0 / factor, 5.0 / factor, 2.0 / factor, 3.0 / factor]];
    //     for i in 0..4 {
    //         relative_eq!(m[[i, 0]], expect.t()[[i, 0]], epsilon = f64::EPSILON);
    //     }
    // }

    // {
    //     let a = arr2(&[[4.0, 5.0, 2.0],
    //                    [4.0, 5.0, 2.0],
    //                    [4.0, 5.0, 2.0]]);
    //     let m : Mat3x3 = a.into();
    // }
}
