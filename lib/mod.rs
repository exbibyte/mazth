pub extern crate ndarray;

#[cfg(test)]
pub extern crate ndarray_rand;

#[macro_use]
#[cfg(test)]
pub extern crate approx;

// pub mod i_intersect;

pub mod constants;
pub mod dualquat;
pub mod dualscalar;
pub mod mat;
pub mod quat;

pub mod bound;
pub mod bound_aabb;

pub mod shape;
pub mod vicinity;
//todo
// pub mod bound_sphere;

pub mod line;
pub mod plane;
pub mod point;
pub mod ray;
pub mod rbox;
pub mod sphere;
pub mod triprism;

pub mod intersect_gjk;

//todo
// #[cfg(test)]
// mod test;
