use crate::{dualquat::*, mat::*, quat::*};

///computes a view matrix by doing an inversion of camera transform to bring world frame to camera frame
pub fn look_at(loc: &Mat3x1, target: &Mat3x1, up: &Mat3x1) -> Mat4x4 {
    let z = (loc-target).normalize_l2();
    let x = up.normalize_l2().cross(&z).normalize_l2();
    let y = z.cross(&x).normalize_l2();
    //inverse of rotation = tranpose
    //does inverse of translation as well
    Mat4x4::new_r([
        x[0],
        x[1],
        x[2],
        -loc.inner(&x),
        y[0],
        y[1],
        y[2],
        -loc.inner(&y),
        z[0],
        z[1],
        z[2],
        -loc.inner(&z),
        0.,
        0.,
        0.,
        1.,
    ])
}

pub fn look_at_non_invert(loc: &Mat3x1, target: &Mat3x1, up: &Mat3x1) -> Mat4x4 {
    let z = (target-loc).normalize_l2();
    let x = up.normalize_l2().cross(&z);
    let y = z.cross(&x);
    Mat4x4::new_r([
        x[0],
        y[0],
        z[0],
        loc.inner(&x),
        x[1],
        y[1],
        z[1],
        loc.inner(&y),
        x[2],
        y[2],
        z[2],
        loc.inner(&z),
        0.,
        0.,
        0.,
        1.,
    ])
}

pub fn look_at_non_invert_quat(loc: &Mat3x1, target: &Mat3x1, up: &Mat3x1) -> DualQuat {
    let m = look_at_non_invert(loc, target, up);
    DualQuat::new(
        Quat::init_from_rotation_matrix(m.sub_rot()),
        Quat::init_from_translation(m.sub_xlate()),
    )
}
