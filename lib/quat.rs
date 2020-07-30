extern crate ndarray;

use ndarray::prelude::*;

#[allow(unused_imports)]
use std::ops::Div;
#[allow(unused_imports)]
use std::ops::Index;
#[allow(unused_imports)]
use std::ops::IndexMut;

use std::f64::consts::PI;

use constants::*;
use mat::*;

use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Quat {
    m: Mat4x1,
}

impl Default for Quat {
    fn default() -> Quat {
        Quat {
            m: Mat4x1::new([0., 0., 0., 1.]),
        }
    }
}

impl Quat {
    pub fn x(&self) -> f64 {
        assert_eq!(self.m.shape()[0], 4);
        self.m[0]
    }
    pub fn y(&self) -> f64 {
        assert_eq!(self.m.shape()[0], 4);
        self.m[1]
    }
    pub fn z(&self) -> f64 {
        assert_eq!(self.m.shape()[0], 4);
        self.m[2]
    }
    pub fn w(&self) -> f64 {
        assert_eq!(self.m.shape()[0], 4);
        self.m[3]
    }
    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.m[0]
    }
    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.m[1]
    }
    pub fn z_mut(&mut self) -> &mut f64 {
        &mut self.m[2]
    }
    pub fn w_mut(&mut self) -> &mut f64 {
        &mut self.m[3]
    }
    #[allow(dead_code)]
    pub fn init_from_vals(x: f64, y: f64, z: f64, w: f64) -> Quat {
        Quat {
            m: Mat4x1::new([x, y, z, w]),
        }
    }

    #[allow(dead_code)]
    pub fn init_from_vals_auto_w(x: f64, y: f64, z: f64) -> Quat {
        let w = 1. - x * x - y * y - z * z;
        if w < 0. {
            Quat {
                m: Mat4x1::new([x, y, z, w]),
            }
        } else {
            Quat {
                m: Mat4x1::new([x, y, z, -1. * w.sqrt()]),
            }
        }
    }
    #[allow(dead_code)]
    pub fn init_from_translation(trans: Mat3x1) -> Quat {
        Quat::init_from_vals(trans[0] / 2., trans[1] / 2., trans[2] / 2., 0.)
    }
    #[allow(dead_code)]
    pub fn to_translation_matrix(&self) -> Mat4x4 {
        //assume current quaternion corresponds to translation
        Mat4x4::new_r([
            0.,
            0.,
            0.,
            2. * self.x(),
            0.,
            0.,
            0.,
            2. * self.y(),
            0.,
            0.,
            0.,
            2. * self.z(),
            0.,
            0.,
            0.,
            1.,
        ])
    }
    ///expects a proper rotation matrix as input
    pub fn init_from_rotation_matrix(rot: Mat3x3) -> Quat {
        let t = rot.trace();
        if t > 0. {
            let s = 0.5 / (t + 1.).sqrt();

            Quat::init_from_vals(
                (rot[[2, 1]] - rot[[1, 2]]) * s,
                (rot[[0, 2]] - rot[[2, 0]]) * s,
                (rot[[1, 0]] - rot[[0, 1]]) * s,
                1. / s * 0.25,
            )
        } else if rot[[0, 0]] > rot[[1, 1]] && rot[[0, 0]] > rot[[2, 2]] {
            let s = 2. * (1. + rot[[0, 0]] - rot[[1, 1]] - rot[[2, 2]]).sqrt();

            Quat::init_from_vals(
                0.25 * s,
                (rot[[0, 1]] + rot[[1, 0]]) / s,
                (rot[[0, 2]] + rot[[2, 0]]) / s,
                (rot[[2, 1]] - rot[[1, 2]]) / s,
            )
        } else if rot[[1, 1]] > rot[[2, 2]] {
            let s = 2. * (1. + rot[[1, 1]] - rot[[0, 0]] - rot[[2, 2]]).sqrt();

            Quat::init_from_vals(
                (rot[[0, 1]] - rot[[1, 0]]) / s,
                0.25 * s,
                (rot[[1, 2]] - rot[[2, 1]]) / s,
                (rot[[0, 2]] - rot[[2, 0]]) / s,
            )
        } else {
            let s = 2. * (1. + rot[[2, 2]] - rot[[0, 0]] - rot[[1, 1]]).sqrt();

            Quat::init_from_vals(
                (rot[[0, 2]] - rot[[2, 0]]) / s,
                (rot[[1, 2]] - rot[[2, 1]]) / s,
                0.25 * s,
                (rot[[1, 0]] - rot[[0, 1]]) / s,
            )
        }
    }
    #[allow(dead_code)]
    pub fn to_rotation_matrix(&self) -> Mat4x4 {
        //assumes unit quaternion
        let a = self.normalize();
        Mat4x4::new_r([
            1. - 2. * (a.y() * a.y() + a.z() * a.z()), //first row
            2. * (a.x() * a.y() - a.z() * a.w()),
            2. * (a.x() * a.z() + a.y() * a.w()),
            0.,
            2. * (a.x() * a.y() + a.z() * a.w()), //second row
            1. - 2. * (a.x() * a.x() + a.z() * a.z()),
            2. * (a.y() * a.z() - a.x() * a.w()),
            0.,
            2. * (a.x() * a.z() - a.y() * a.w()), //third row
            2. * (a.z() * a.y() + a.x() * a.w()),
            1. - 2. * (a.x() * a.x() + a.y() * a.y()),
            0.,
            0.,
            0.,
            0.,
            1., //last row
        ])
    }
    #[allow(dead_code)]
    pub fn init_from_axis_angle_degree(axis: Mat3x1, angle: f64) -> Quat {
        Self::init_from_axis_angle_radian(axis, angle / 180. * PI)
    }
    #[allow(dead_code)]
    pub fn init_from_axis_angle_radian(axis: Mat3x1, angle: f64) -> Quat {
        let radian = ((angle % (2. * PI)) + 2. * PI) % (2. * PI);
        let axis_adjust = axis.normalize_l2();
        let sine_half = (radian / 2.).sin();
        Quat::init_from_vals(
            axis_adjust[0] * sine_half,
            axis_adjust[1] * sine_half,
            axis_adjust[2] * sine_half,
            (radian / 2.).cos(),
        )
    }
    ///returns [x,y,z], angle where angle is in radian
    #[allow(dead_code)]
    pub fn to_axis_angle(&self) -> (Mat3x1, f64) {
        let q = self.normalize();
        let k = (1. - q.w() * q.w()).sqrt();
        if k < EPS {
            (Mat3x1::new([1., 0., 0.]), 0.)
        } else {
            let vec_x = q.x() / k;
            let vec_y = q.y() / k;
            let vec_z = q.z() / k;
            (Mat3x1::new([vec_x, vec_y, vec_z]), 2. * self.w().acos())
        }
    }
    ///rotation of a vector p, by a unit quaternion q:  q * p q', where q' is the conjugate
    #[allow(dead_code)]
    pub fn rotate_vector(&self, p: Mat3x1) -> Mat3x1 {
        let quat_p = Quat::init_from_vals(p[0], p[1], p[2], 0.);
        let temp2 = &(self * &quat_p) * &self.conjugate();
        Mat3x1::new([temp2.x(), temp2.y(), temp2.z()])
    }
    #[allow(dead_code)]
    pub fn reflection_in_plane(&self, p: Mat3x1) -> Mat3x1 {
        let quat_p = Quat::init_from_vals(p[0], p[1], p[2], 0.);
        let temp = self * &quat_p;
        let temp2 = &temp * self;
        Mat3x1::new([temp2.x(), temp2.y(), temp2.z()])
    }
    #[allow(dead_code)]
    pub fn parallel_component_of_plane(&self, p: Mat3x1) -> Mat3x1 {
        let quat_p = Quat::init_from_vals(p[0], p[1], p[2], 0.);
        let temp = self * &quat_p;
        let temp2 = &temp * self;
        let temp3 = &quat_p + &temp2;
        let temp4 = 0.5 * &temp3;
        Mat3x1::new([temp4.x(), temp4.y(), temp4.z()])
    }
    #[allow(dead_code)]
    pub fn orthogonal_component_of_plane(&self, p: Mat3x1) -> Mat3x1 {
        let quat_p = Quat::init_from_vals(p[0], p[1], p[2], 0.);
        let temp = self * &quat_p;
        let temp2 = &temp * self;
        let temp3 = &quat_p - &temp2;
        let temp4 = 0.5 * &temp3;
        Mat3x1::new([temp4.x(), temp4.y(), temp4.z()])
    }
    #[allow(dead_code)]
    pub fn add(&self, other: &Self) -> Quat {
        Quat::init_from_vals(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
            self.w() + other.w(),
        )
    }
    #[allow(dead_code)]
    pub fn minus(&self, other: &Self) -> Quat {
        Quat::init_from_vals(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
            self.w() - other.w(),
        )
    }
    #[allow(dead_code)]
    pub fn mul(&self, other: &Self) -> Quat {
        Quat::init_from_vals(
            self.w() * other.x() + self.x() * other.w() + self.y() * other.z()
                - self.z() * other.y(),
            self.w() * other.y() - self.x() * other.z()
                + self.y() * other.w()
                + self.z() * other.x(),
            self.w() * other.z() + self.x() * other.y() - self.y() * other.x()
                + self.z() * other.w(),
            self.w() * other.w()
                - self.x() * other.x()
                - self.y() * other.y()
                - self.z() * other.z(),
        )
    }
    #[allow(dead_code)]
    pub fn norm_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z() + self.w() * self.w()
    }
    #[allow(dead_code)]
    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }
    #[allow(dead_code)]
    pub fn normalize(&self) -> Quat {
        let l = self.norm();
        if l > 0. || l < 0. {
            Quat::init_from_vals(self.x() / l, self.y() / l, self.z() / l, self.w() / l)
        } else {
            panic!("quat normalization unsuccessful.");
        }
    }
    #[allow(dead_code)]
    pub fn normalized(&mut self) {
        let l = self.norm();
        if l > 0. || l < 0. {
            *self.x_mut() = self.x() / l;
            *self.y_mut() = self.y() / l;
            *self.z_mut() = self.z() / l;
            *self.w_mut() = self.w() / l;
        } else {
            panic!("quat normalization unsuccessful.");
        }
    }
    #[allow(dead_code)]
    pub fn ln(&self) -> Quat {
        let l = self.norm();
        let w_ln = self.w().ln();
        //normalize x,y,z vector -> v/||v||
        let vec_length = (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt();
        assert!(vec_length != 0.);
        let vec_x = self.x() / vec_length;
        let vec_y = self.y() / vec_length;
        let vec_z = self.z() / vec_length;
        //scale x,y,z by acos( w/l )
        let s = (w_ln / l).acos();
        Quat::init_from_vals(vec_x * s, vec_y * s, vec_z * s, w_ln)
    }
    #[allow(dead_code)]
    pub fn pow(&self, t: f64) -> Quat {
        let vec_length = (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt();
        assert!(vec_length != 0.);
        let vec_x = self.x() / vec_length;
        let vec_y = self.y() / vec_length;
        let vec_z = self.z() / vec_length;
        let l = self.norm();
        //original angle
        let alpha = (self.w() / l).acos();
        //new angle
        let beta = t * alpha;
        let coeff = l.powf(t);
        Quat::init_from_vals(
            coeff * vec_x * beta.sin(),
            coeff * vec_y * beta.sin(),
            coeff * vec_z * beta.sin(),
            coeff * beta.cos(),
        )
    }
    #[allow(dead_code)]
    pub fn negate(&self) -> Quat {
        Quat::init_from_vals(-self.x(), -self.y(), -self.z(), -self.w())
    }
    #[allow(dead_code)]
    pub fn conjugate(&self) -> Quat {
        Quat::init_from_vals(-self.x(), -self.y(), -self.z(), self.w())
    }
    #[allow(dead_code)]
    pub fn scale(&self, s: f64) -> Quat {
        Quat::init_from_vals(self.x() * s, self.y() * s, self.z() * s, self.w() * s)
    }
    #[allow(dead_code)]
    pub fn scaled(&mut self, s: f64) {
        *self.x_mut() = self.x() * s;
        *self.y_mut() = self.y() * s;
        *self.z_mut() = self.z() * s;
        *self.w_mut() = self.w() * s;
    }
    #[allow(dead_code)]
    pub fn inverse(&self) -> Quat {
        let conj = self.conjugate();
        let norm = conj.norm_squared();
        assert!(norm != 0.);
        (1. / norm) * &conj
    }
    #[allow(dead_code)]
    pub fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z() + self.w() * other.w()
    }
    #[allow(dead_code)]
    pub fn interpolate_linear(start: Quat, end: Quat, t: f64) -> Quat {
        let clamp_upper = if t > 1. { 1. } else { t };
        let clamp = if clamp_upper < 0. { 0. } else { clamp_upper };
        Quat::init_from_vals(
            start.x() * (1. - clamp) + end.x() * clamp,
            start.y() * (1. - clamp) + end.y() * clamp,
            start.z() * (1. - clamp) + end.z() * clamp,
            start.w() * (1. - clamp) + end.w() * clamp,
        )
    }
    #[allow(dead_code)]
    pub fn interpolate_slerp(start: Quat, end: Quat, t: f64) -> Quat {
        let t_clamp_upper = if t > 1. { 1. } else { t };
        let t_clamp = if t_clamp_upper < 0. {
            0.
        } else {
            t_clamp_upper
        };

        let cos_omega =
            start.w() * end.w() + start.x() * end.x() + start.y() * end.y() + start.z() * end.z();
        let cos_omega_adjust = if cos_omega < 0. {
            -cos_omega
        } else {
            cos_omega
        };

        let end_adjust = if cos_omega < 0. {
            //inverted
            Quat::init_from_vals(-end.x(), -end.y(), -end.z(), -end.w())
        } else {
            Quat::init_from_vals(end.x(), end.y(), end.z(), end.w())
        };

        let (k0, k1) = if cos_omega_adjust > 0.9999 {
            (1. - t_clamp, t_clamp)
        } else {
            let sin_omega = (1. - cos_omega * cos_omega).sqrt();
            let omega = sin_omega.atan2(cos_omega);
            let inv_sin_omega = 1. / sin_omega;
            (
                ((1. - t_clamp) * omega).sin() * inv_sin_omega,
                (t_clamp * omega).sin() * inv_sin_omega,
            )
        };
        Quat::init_from_vals(
            start.x() * k0 + end_adjust.x() * k1,
            start.y() * k0 + end_adjust.y() * k1,
            start.z() * k0 + end_adjust.z() * k1,
            start.w() * k0 + end_adjust.w() * k1,
        )
    }
}

impl Add for &Quat {
    type Output = Quat;
    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl<'a> Mul<&'a Quat> for &'a Quat {
    type Output = Quat;
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(rhs)
    }
}

impl Mul<f64> for &Quat {
    type Output = Quat;
    fn mul(self, rhs: f64) -> Self::Output {
        self.scale(rhs)
    }
}

impl Mul<&Quat> for f64 {
    type Output = Quat;
    fn mul(self, rhs: &Quat) -> Quat {
        rhs.scale(self)
    }
}

impl Sub for &Quat {
    type Output = Quat;
    fn sub(self, rhs: Self) -> Self::Output {
        self.minus(rhs)
    }
}
