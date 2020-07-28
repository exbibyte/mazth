use std::f64;

use i_bound::Bound;
use i_bound::BoundType;
use i_shape::ShapeType;

#[derive(Debug, Clone)]
pub struct BoundSphere {
    //todo
}

impl BoundSphere {
    pub fn init(_shape_type: ShapeType, _vals: &[f64]) -> BoundSphere {
        BoundSphere {}
    }
}

impl Bound for BoundSphere {
    fn get_type(&self) -> BoundType {
        BoundType::Sphere
    }
    fn intersect(&self, _other: &dyn Bound) -> bool {
        unimplemented!();
    }
    fn get_shortest_separation(&self, _other: &dyn Bound) -> f64 {
        unimplemented!();
    }
    fn get_bound_data(&self) -> [f64; 32] {
        unimplemented!();
    }
    fn get_union(&mut self, _bounds: &[&dyn Bound]) {
        unimplemented!();
    }
    fn get_centroid(&self) -> [f64; 3] {
        unimplemented!();
    }
}

impl Default for BoundSphere {
    fn default() -> BoundSphere {
        unimplemented!();
    }
}
