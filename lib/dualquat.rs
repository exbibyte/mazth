#[allow(unused_imports)]
use std::ops::Div;
#[allow(unused_imports)]
use std::ops::Index;
#[allow(unused_imports)]
use std::ops::IndexMut;
#[allow(unused_imports)]
use std::ops::{Add, Mul, Sub};

use constants::*;
use dualscalar::*;
use mat::*;
use quat::*;

use ndarray::arr1;

///(rot, translation) pair
#[derive(Debug, Clone, Copy)]
pub struct DualQuat(Quat, Quat);

impl DualQuat {
    #[allow(dead_code)]
    pub fn quat_rot(&self) -> &Quat {
        &self.0
    }
    #[allow(dead_code)]
    pub fn quat_tra(&self) -> &Quat {
        &self.1
    }
    pub fn quat_rot_mut(&mut self) -> &mut Quat {
        &mut self.0
    }
    #[allow(dead_code)]
    pub fn quat_tra_mut(&mut self) -> &mut Quat {
        &mut self.1
    }
    #[allow(dead_code)]
    pub fn dual_scalar(&self) -> DualScalar {
        DualScalar::new(self.quat_rot().w(), self.quat_tra().w())
    }
    #[allow(dead_code)]
    pub fn new_from_rot(rotate: Quat) -> DualQuat {
        DualQuat(rotate.normalize(), Quat::init_from_vals(0., 0., 0., 0.))
    }
    #[allow(dead_code)]
    pub fn new_from_tra(translate: Quat) -> DualQuat {
        DualQuat(Quat::init_from_vals(0., 0., 0., 1.0), translate)
    }
    #[allow(dead_code)]
    pub fn new(rotate: Quat, translate: Quat) -> DualQuat {
        let t = &translate * &rotate;
        DualQuat(rotate, t)
    }
    #[allow(dead_code)]
    pub fn new_raw(rotate: Quat, translate: Quat) -> DualQuat {
        DualQuat(rotate, translate)
    }
    ///returns 4x4 homogeneous matrix
    pub fn xform_rot(&self) -> Mat4x4 {
        self.normalize().quat_rot().to_rotation_matrix()
    }
    ///returns vec4
    pub fn xform_tra(&self) -> Mat4x1 {
        let a = self.normalize();
        // let b = 2. * &(&a.quat_rot().conjugate() * a.quat_tra());
        let b = &(2. * a.quat_tra()) * &a.quat_rot().conjugate();
        Mat4x1::new([b.x(), b.y(), b.z(), 1.])
    }
    ///returns 4x4 homogeneous matrix
    pub fn xform(&self) -> Mat4x4 {
        let a = self.xform_tra();
        let mut b = self.xform_rot();
        b[[0, 3]] = a[0];
        b[[1, 3]] = a[1];
        b[[2, 3]] = a[2];
        b
    }
    pub fn normalize(&self) -> DualQuat {
        // dbg!(self);
        let l = self.quat_rot().norm();
        assert!(l > EPS);
        let a = self.quat_rot().scale(1. / l);
        let b = self.quat_tra().scale(1. / l);
        DualQuat(a, b)
    }
    pub fn normalized(&mut self) {
        let (a, b) = <(Quat, Quat)>::from(self.normalize());
        *self.quat_rot_mut() = a;
        *self.quat_tra_mut() = b;
    }
    pub fn norm(&self) -> DualScalar {
        DualScalar::new(
            self.quat_rot().norm(),
            self.quat_rot().dot(self.quat_tra()) / self.quat_rot().norm(),
        )
    }
    pub fn conjugate(&self) -> DualQuat {
        DualQuat(self.quat_rot().conjugate(), self.quat_tra().conjugate())
    }
    pub fn rotate_point(&self, p: Mat3x1) -> Mat3x1 {
        let t =
            &(self * &DualQuat::new_from_tra(Quat::init_from_translation(p))) * &self.conjugate();
        t.xform_tra().vec3()
    }
    /// q(q* p)^t
    pub fn sclerp(&self, other: &Self, t: f64) -> DualQuat {
        // self * (&(&self.conjugate() * other).pow(t))

        // take shortest
        let dot = self.quat_rot().dot(other.quat_rot());
        let o;
        if dot < 0. {
            o = DualQuat::new_raw(other.quat_rot() * -1., other.quat_tra() * -1.);
        } else {
            o = other.clone();
        }

        let diff = &self.conjugate() * &o;

        self * &diff.pow(t)
    }
    fn pow(&self, e: f64) -> DualQuat {
        let vr = Mat3x1::new([
            self.quat_rot().x(),
            self.quat_rot().y(),
            self.quat_rot().z(),
        ]);

        let vd = Mat3x1::new([
            self.quat_tra().x(),
            self.quat_tra().y(),
            self.quat_tra().z(),
        ]);

        let mut invr = 1. / vr.norm_l2();
        // dbg!(&diff);
        // dbg!(&invr);
        if invr.is_infinite() {
            //pure translation
            let mut q = self.clone();
            *q.quat_tra_mut().x_mut() = q.quat_tra().x() * e;
            *q.quat_tra_mut().y_mut() = q.quat_tra().y() * e;
            *q.quat_tra_mut().z_mut() = q.quat_tra().z() * e;
            return q.normalize();
        }

        // screw
        let mut angle = 2. * self.quat_rot().w().acos();
        let mut pitch = -2. * self.quat_tra().w() * invr;
        let direction = &vr * invr;

        // dbg!(&direction);
        let moment = &(&vd - &(&(&direction * pitch) * (&self.quat_rot().w() * 0.5))) * invr;

        angle *= e;
        pitch *= e;

        //convert back
        let sinAngle = (0.5 * angle).sin();
        let cosAngle = (0.5 * angle).cos();
        let temp = &direction * sinAngle;
        let real = Quat::init_from_vals(temp[0], temp[1], temp[2], cosAngle);

        let temp2 = &(sinAngle * &moment) + &(pitch * 0.5 * cosAngle * &direction);
        let dual = Quat::init_from_vals(temp2[0], temp2[1], temp2[2], -pitch * 0.5 * sinAngle);

        DualQuat(real, dual)
    }
    // ///todo: debug this function
    // fn pow(&self, e: f64) -> DualQuat {
    //     let mut d = self.clone();

    //     let mut screwaxis = Matrix1D::from(arr1(&[0., 0., 0.]));
    //     let mut moment = Matrix1D::from(arr1(&[0., 0., 0.]));
    //     let mut angles = Matrix1D::from(arr1(&[0., 0.]));

    //     let norm_a = d.get_screw_parameters(&mut screwaxis, &mut moment, &mut angles);

    //     // pure translation
    //     if norm_a < EPS {
    //         println!("pure translation");
    //         *d.quat_tra_mut().x_mut() = d.quat_tra().x() * e;
    //         *d.quat_tra_mut().y_mut() = d.quat_tra().y() * e;
    //         *d.quat_tra_mut().z_mut() = d.quat_tra().z() * e;
    //         d.normalized();
    //         d
    //     } else {
    //         // exponentiate
    //         let theta = angles[0] * e;
    //         let alpha = angles[1] * e;
    //         println!("theta, alpha: {:?}, {:?}", &theta, &alpha);
    //         // convert back
    //         d.set_screw_parameters(screwaxis.view(), moment.view(), theta, alpha);
    //         d
    //     }
    // }
    fn get_screw_parameters(
        &self,
        screwaxis: &mut Matrix1D,
        moment: &mut Matrix1D,
        angles: &mut Matrix1D,
    ) -> f64 {
        let q_a = Matrix1D::from(arr1(&[
            self.quat_rot().x(),
            self.quat_rot().y(),
            self.quat_rot().z(),
        ]));

        let q_b = Matrix1D::from(arr1(&[
            self.quat_tra().x(),
            self.quat_tra().y(),
            self.quat_tra().z(),
        ]));

        let norm_a = q_a.view().norm_l2();

        // pure translation
        if norm_a < EPS {
            println!("pure translation");
            let norm_a = q_b.norm_l2();
            *screwaxis = q_b.normalize_l2();

            for i in 0..3 {
                moment[i] = 0.;
            }
            angles[0] = 0.;
            angles[1] = 2. * q_b.norm_l2();
            norm_a
        } else {
            *screwaxis = q_a.normalize_l2();
            angles[0] = 2. * norm_a.atan2(self.quat_rot().w());
            // if (angles[0] > Math.PI / 2) {
            //    angles[0] -= Math.PI;
            // }
            angles[1] = -2. * self.quat_tra().w() / norm_a;
            let m1 = 1. / norm_a * q_b;
            let m2 =
                self.quat_rot().w() * self.quat_tra().w() / (norm_a * norm_a) * screwaxis.clone();
            *moment = &m1 + &m2;
            norm_a
        }
    }

    fn set_screw_parameters(
        &mut self,
        screwaxis: Matrix1DView,
        moment: Matrix1DView,
        theta: f64,
        alpha: f64,
    ) {
        let cosa = (theta / 2.).cos();
        let sina = (theta / 2.).sin();

        *self.quat_rot_mut().w_mut() = cosa;
        *self.quat_rot_mut().x_mut() = sina * screwaxis[0];
        *self.quat_rot_mut().x_mut() = sina * screwaxis[1];
        *self.quat_rot_mut().x_mut() = sina * screwaxis[2];

        *self.quat_tra_mut().w_mut() = -alpha / 2. * sina;
        *self.quat_tra_mut().x_mut() = sina * moment[0] + alpha / 2. * cosa * screwaxis[0];
        *self.quat_tra_mut().y_mut() = sina * moment[1] + alpha / 2. * cosa * screwaxis[1];
        *self.quat_tra_mut().z_mut() = sina * moment[2] + alpha / 2. * cosa * screwaxis[2];

        self.normalized();
    }
}

///useful for transforms, eg: p_new = q*p*q', q := transform, q' := conjugate, p := vector point in dualquat
impl Mul for &DualQuat {
    type Output = DualQuat;
    fn mul(self, rhs: &DualQuat) -> DualQuat {
        let a = self;
        let b = rhs;
        DualQuat(
            a.quat_rot().mul(b.quat_rot()),
            &a.quat_tra().mul(b.quat_rot()) + &a.quat_rot().mul(b.quat_tra()),
        )
    }
}

impl Add for &DualQuat {
    type Output = DualQuat;
    fn add(self, rhs: &DualQuat) -> DualQuat {
        DualQuat(
            self.quat_rot() + rhs.quat_rot(),
            self.quat_tra() + rhs.quat_tra(),
        )
    }
}

impl Sub for &DualQuat {
    type Output = DualQuat;
    fn sub(self, rhs: &DualQuat) -> DualQuat {
        DualQuat(
            self.quat_rot() - rhs.quat_rot(),
            self.quat_tra() - rhs.quat_tra(),
        )
    }
}

impl From<DualQuat> for (Quat, Quat) {
    fn from(i: DualQuat) -> (Quat, Quat) {
        (i.0, i.1)
    }
}
