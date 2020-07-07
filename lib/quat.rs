extern crate ndarray;

use ndarray::prelude::*;
use ndarray::{arr2, aview0, aview1, Axis};
use ndarray::{Array, Ix3};

#[allow(unused_imports)]
use std::ops::Div;
#[allow(unused_imports)]
use std::ops::Index;
#[allow(unused_imports)]
use std::ops::IndexMut;
#[allow(unused_imports)]
use std::ops::Mul;

use std::f64::consts::PI;

use crate::mat::*;

#[derive(Debug, Clone)]
pub struct Quat {
    ///data: [x,y,z,w]
    m: Matrix1D,
}

impl Default for Quat {
    fn default() -> Quat {
        Quat {
            m: array![0., 0., 0., 1.],
        }
    }
}

impl Quat {
    pub fn x(&self) -> f64 {
        self.m[0]
    }
    pub fn y(&self) -> f64 {
        self.m[1]
    }
    pub fn z(&self) -> f64 {
        self.m[2]
    }
    pub fn w(&self) -> f64 {
        self.m[3]
    }

    #[allow(dead_code)]
    pub fn init_from_vals(x: f64, y: f64, z: f64, w: f64) -> Quat {
        Quat {
            m: array![x, y, z, w],
        }
    }

    #[allow(dead_code)]
    pub fn init_from_vals_auto_w(x: f64, y: f64, z: f64) -> Quat {
        let w = 1. - x * x - y * y - z * z;
        if w < 0. {
            Quat {
                m: array![x, y, z, w],
            }
        } else {
            Quat {
                m: array![x, y, z, -1. * w.sqrt()],
            }
        }
    }
    #[allow(dead_code)]
    pub fn init_from_translation(trans: Matrix) -> Quat {
        assert!(trans.shape().len() == 2);
        assert!(trans.shape()[0] == 3);
        assert!(trans.shape()[1] == 1);
        Quat {
            m: array![
                trans[[0, 0]] / 2.,
                trans[[1, 0]] / 2.,
                trans[[2, 0]] / 2.,
                0.
            ],
        }
    }

    #[allow(dead_code)]
    pub fn to_translation_matrix(&self, row_major: bool) -> Matrix {
        array![
            [0., 0., 0., self.x()],
            [0., 0., 0., self.y()],
            [0., 0., 0., self.z()],
            [0., 0., 0., 1.]
        ]
    }

    #[allow(dead_code)]
    pub fn to_rotation_matrix(&self, row_major: bool) -> Matrix {
        //assumes unit quaternion
        array![
            [
                1. - 2. * (self.y() * self.y() + self.z() * self.z()), //first row
                2. * (self.x() * self.y() - self.z() * self.w()),
                2. * (self.x() * self.z() + self.y() * self.w()),
                0.
            ],
            [
                2. * (self.x() * self.y() + self.z() * self.w()), //second row
                1. - 2. * (self.x() * self.x() + self.z() * self.z()),
                2. * (self.y() * self.z() - self.x() * self.w()),
                0.
            ],
            [
                2. * (self.x() * self.z() - self.y() * self.w()), //third row
                2. * (self.z() * self.y() + self.x() * self.w()),
                1. - 2. * (self.x() * self.x() + self.y() * self.y()),
                0.
            ],
            [0., 0., 0., 1.]
        ]
    }
    #[allow(dead_code)]
    pub fn to_axis_angle(&self) -> Matrix {
        ///returns [x,y,z,angle]
        let k = (1. - self.w() * self.w()).sqrt();
        if k < eps {
            array![[1.], [0.], [0.], [0.]]
        } else {
            let vec_x = self.x() / k;
            let vec_y = self.y() / k;
            let vec_z = self.z() / k;
            let l = (vec_x * vec_x + vec_y * vec_y + vec_z * vec_z).sqrt();
            // assert!(l.abs()>eps);
            array![[vec_x / l], [vec_y / l], [vec_z / l], [2. * self.w().acos()]]
        }
    }
    #[allow(dead_code)]
    pub fn init_from_axis_angle_degree(axis_angle: MatrixView) -> Quat {
        let angle = axis_angle[[3, 0]];
        let axis = axis_angle.slice(s![0..3, ..]);
        let radian = angle / 180. * PI;
        let s = array![[axis[[0, 0]]], [axis[[1, 0]]], [axis[[2, 0]]], [radian]];
        Self::init_from_axis_angle_radian(s.t())
    }
    #[allow(dead_code)]
    pub fn init_from_axis_angle_radian(axis_angle: MatrixView) -> Quat {
        let radian = axis_angle[[3, 0]];
        let axis = axis_angle.slice(s![0..3, ..]);
        let axis_adjust = normalize_vec_l2(&axis);
        let sine_half = (radian / 2.).sin();
        Quat::init_from_vals(axis_adjust[[0, 0]] * sine_half,
                             axis_adjust[[1, 0]] * sine_half,
                             axis_adjust[[2, 0]] * sine_half,
                             (radian / 2.).cos())
    }
    #[allow(dead_code)]
    pub fn rotate_vector(&self, p: MatrixView) -> Matrix {
        let conj = self.conjugate();
        let quat_p = Quat::init_from_vals(p[[0, 0]], p[[1, 0]], p[[2, 0]], 0.);
        let temp = self.mul(&quat_p);
        let temp2 = temp.mul(&conj);
        array![[temp2.x()], [temp2.y()], [temp2.z()]]
    }
    #[allow(dead_code)]
    pub fn reflection_in_plane(&self, p: MatrixView) -> Matrix {
        let quat_p = Quat::init_from_vals(p[[0, 0]], p[[1, 0]], p[[2, 0]], 0.);
        let temp = self.mul(&quat_p);
        let temp2 = temp.mul(self);
        array![[temp2.x()], [temp2.y()], [temp2.z()]]
    }
    #[allow(dead_code)]
    pub fn parallel_component_of_plane(&self, p: MatrixView) -> Matrix {
        let quat_p = Quat::init_from_vals(p[[0, 0]], p[[1, 0]], p[[2, 0]], 0.);
        let temp = self.mul(&quat_p);
        let temp2 = temp.mul(self);
        let temp3 = quat_p.add(&temp2);
        let temp4 = temp3.scale(0.5);
        array![[temp4.x()], [temp4.y()], [temp4.z()]]
    }
    #[allow(dead_code)]
    pub fn orthogonal_component_of_plane(&self, p: MatrixView) -> Matrix {
        let quat_p = Quat::init_from_vals(p[[0, 0]], p[[1, 0]], p[[2, 0]], 0.);
        let temp = self.mul(&quat_p);
        let temp2 = temp.mul(self);
        let temp3 = quat_p.minus(&temp2);
        let temp4 = temp3.scale(0.5);
        array![[temp4.x()], [temp4.y()], [temp4.z()]]
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
    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z() + self.w() * self.w()
    }
    #[allow(dead_code)]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    #[allow(dead_code)]
    pub fn normalize(&self) -> Quat {
        let l = self.length();
        if l > 0. || l < 0. {
            Quat::init_from_vals(self.x() / l, self.y() / l, self.z() / l, self.w() / l)
        } else {
            panic!("quat normalization unsuccessful.");
        }
    }
    #[allow(dead_code)]
    pub fn ln(&self) -> Quat {
        let l = self.length();
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
        let l = self.length();
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
    pub fn inverse(&self) -> Quat {
        let conj = self.conjugate();
        let norm = conj.length_squared();
        assert!(norm != 0.);
        conj.scale(1. / norm)
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
