pub extern crate ndarray;

pub extern crate ndarray_linalg;

#[cfg(test)]
pub extern crate ndarray_rand;

#[macro_use]
#[cfg(test)]
pub extern crate approx;

pub mod constants;

pub mod dualquat;
pub mod dualscalar;
pub mod mat;
pub mod quat;

pub mod bound;
pub mod bound_aabb;
pub mod intersect_gjk;
pub mod line;
pub mod plane;
pub mod point;
pub mod ray;
pub mod rbox;
pub mod shape;
pub mod sphere;
pub mod triprism;
pub mod vicinity;

pub mod util_graphics;

//todo
// pub mod bound_sphere;

#[cfg(test)]
mod test;
