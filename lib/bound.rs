pub enum BoundType {
    AxisAlignBox,
    Sphere,
}

pub trait Bound {
    fn get_type(&self) -> BoundType;
    fn intersect(&self, other: &dyn Bound) -> bool;
    fn get_shortest_separation(&self, other: &dyn Bound) -> f64;
    fn get_bound_data(&self) -> [f64; 32];
    fn get_union(&mut self, bounds: &[&dyn Bound]);
    fn get_centroid(&self) -> [f64; 3];
}
