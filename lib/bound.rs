pub enum BoundType {
    AxisAlignBox,
    Sphere,
}

pub trait IBound {
    fn get_type(&self) -> BoundType;
    fn intersect(&self, other: &dyn IBound) -> bool;
    fn get_shortest_separation(&self, other: &dyn IBound) -> f64;
    fn get_bound_data(&self) -> [f64; 32];
    fn get_union(&mut self, bounds: &[&dyn IBound]);
    fn get_centroid(&self) -> [f64; 3];
}
