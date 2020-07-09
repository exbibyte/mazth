#[allow(unused_imports)]
use std::ops::Div;
#[allow(unused_imports)]
use std::ops::Index;
#[allow(unused_imports)]
use std::ops::IndexMut;
#[allow(unused_imports)]
use std::ops::{Add,Sub,Mul};

use mat::*;
use quat::Quat;

use ndarray::prelude::*;
use ndarray::{arr1, arr2, aview0, aview1, Axis};
use ndarray::{Array, Ix3};

///(rot, translation) pair
#[derive(Debug, Clone)]
pub struct DualQuat(Quat, Quat);

impl DualQuat {
    #[allow(dead_code)]
    pub fn new_from_rot(rotate: Quat) -> DualQuat {
        DualQuat(rotate, Quat::init_from_vals(0., 0., 0., 0.))
    }
    #[allow(dead_code)]
    pub fn new_from_tra(translate: Quat) -> DualQuat {
        DualQuat(Quat::init_from_vals(0., 0., 0., 1.0), translate)
    }
    #[allow(dead_code)]
    pub fn new(rotate: Quat, translate: Quat) -> DualQuat {
        DualQuat(rotate, translate)
    }
    #[allow(dead_code)]
    pub fn quat_rot(&self) -> &Quat {
        &self.0
    }
    #[allow(dead_code)]
    pub fn quat_tra(&self) -> &Quat {
        &self.1
    }
    pub fn xform_rot(&self) -> Matrix {
        self.1.to_rotation_matrix()
    }
    pub fn xform_tra(&self) -> Matrix1D {
        let a = self.1.scale(2.0).mul(&self.0.conjugate());
        arr1(&[a.x(), a.y(), a.z()])
    }
    pub fn xform(&self) -> Matrix {
        let a = self.xform_tra();
        let mut b = self.xform_rot();
        b[[0, 3]] = a[0];
        b[[1, 3]] = a[1];
        b[[2, 3]] = a[2];
        b
    }
    pub fn normalize(&self) -> DualQuat {
        let l = self.quat_rot().length();
        assert!(l > eps);
        DualQuat::new(self.quat_rot().scale(l), self.quat_tra().scale(l))
    }
    pub fn normalized(&mut self) {
        let l = self.quat_rot().length();
        assert!(l > eps);
        self.0.scaled(l);
        self.1.scaled(l);
    }
}

impl Mul for DualQuat {
    type Output = DualQuat;
    fn mul(self, rhs: DualQuat) -> DualQuat {
        DualQuat::new(
            self.quat_rot().mul(rhs.quat_rot()),
            &self.quat_tra().mul(rhs.quat_rot()) + &self.quat_rot().mul(rhs.quat_tra()),
        )
    }
}

impl Add for DualQuat {
    type Output = DualQuat;
    fn add(self, rhs: DualQuat) -> DualQuat {
        DualQuat::new(
            self.quat_rot() + rhs.quat_rot(),
            self.quat_tra() + rhs.quat_tra(),
        )
    }
}

impl Sub for DualQuat {
    type Output = DualQuat;
    fn sub(self, rhs: DualQuat) -> DualQuat {
        DualQuat::new(
            self.quat_rot() - rhs.quat_rot(),
            self.quat_tra() - rhs.quat_tra(),
        )
    }
}
