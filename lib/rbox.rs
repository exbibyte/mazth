use ndarray::prelude::*;

use bound::IBound;
use shape::{IShape, ShapeType};
use vicinity::IVicinity;

use bound_aabb::AxisAlignedBBox;
use mat::*;

#[derive(Debug, Clone)]
pub struct RecBox {
    pub _ori: Matrix1D,
    pub _size: f64,
    pub _bound: AxisAlignedBBox,
    pub _vicinity: f64,
}

impl RecBox {
    pub fn init(origin: &[f64], size: f64) -> RecBox {
        assert!(origin.len() == 3);
        RecBox {
            _ori: arr1(&[origin[0], origin[1], origin[2]]),
            _size: size, //half of the length of box edge
            _bound: AxisAlignedBBox::init(ShapeType::Box, &[&origin[0..3], &[size]].concat()),
            _vicinity: 0.000001f64,
        }
    }
}

impl IShape for RecBox {
    fn get_shape_data(&self) -> Vec<f64> {
        vec![self._ori[0], self._ori[1], self._ori[2], self._size]
    }
    fn get_type(&self) -> ShapeType {
        ShapeType::Box
    }
    fn get_bound(&self) -> &dyn IBound {
        &self._bound
    }
    // this shall test for intersection of bounding shapes first before procedding to test intersection using algorithms of higher complexity
    fn get_intersect(&self, other: &dyn IShape) -> (bool, Option<Matrix1D>) {
        if !self.get_bound().intersect(other.get_bound()) {
            return (false, None);
        } else {
            match other.get_type() {
                ShapeType::Point => {
                    //covered by bbox test
                    let other_shape_data = other.get_shape_data();
                    let b_off = arr1(&[
                        other_shape_data[0],
                        other_shape_data[1],
                        other_shape_data[2],
                    ]);
                    return (true, Some(b_off));
                }
                _ => {
                    unimplemented!();
                }
            }
        }
    }
    fn get_support(&self, v: &Matrix1D) -> Option<Matrix1D> {
        if mag_vec_l2_1d(&v.view()) > 0.000_001f64 {
            //get a furthest point in the given direction v
            let points = [
                arr1(&[self._size, self._size, self._size]),
                arr1(&[-self._size, self._size, self._size]),
                arr1(&[self._size, -self._size, self._size]),
                arr1(&[-self._size, -self._size, self._size]),
                arr1(&[self._size, self._size, -self._size]),
                arr1(&[-self._size, self._size, -self._size]),
                arr1(&[self._size, -self._size, -self._size]),
                arr1(&[-self._size, -self._size, -self._size]),
            ];

            let furthest = points
                .iter()
                .map(|x| x.dot(v))
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            let o = &self._ori + &points[furthest.0];
            Some(o)
        } else {
            None
        }
    }
}

impl IVicinity<f64> for RecBox {
    fn set_vicinity(&mut self, epsilon: f64) {
        self._vicinity = epsilon.abs();
    }
    fn within_vicinity(&self, a: f64, b: f64) -> bool {
        if a + self._vicinity >= b && a - self._vicinity <= b {
            true
        } else {
            false
        }
    }
}
