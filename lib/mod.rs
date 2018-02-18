pub mod i_comparable;
pub mod i_bound;
pub mod i_shape;
pub mod i_vicinity;
pub mod i_intersect;

pub mod quat;
pub mod dualquat;
pub mod mat;

pub mod bound;
pub mod bound_sphere;

///shape implementations
pub mod ray;
pub mod point;
pub mod sphere;
pub mod plane;

pub mod intersect_gjk;

#[cfg(test)]
mod test;

