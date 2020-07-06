#[allow(unused_imports)]
use std::ops::Div;
#[allow(unused_imports)]
use std::ops::Index;
#[allow(unused_imports)]
use std::ops::IndexMut;
#[allow(unused_imports)]
use std::ops::Mul;

use quat::Quat;

#[derive(Debug, Clone)]
pub struct DualQuat(Quat, Quat);

impl DualQuat {
    #[allow(dead_code)]
    pub fn new(rotate: Quat, translate: Quat) -> DualQuat {
        DualQuat(rotate, translate)
    }
    #[allow(dead_code)]
    pub fn rot(&self) -> &Quat {
        &self.0
    }
    #[allow(dead_code)]
    pub fn tra(&self) -> &Quat {
        &self.1
    }
}
