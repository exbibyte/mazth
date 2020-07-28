use bound::Bound;
use mat::*;
use vicinity::Vicinity;

#[derive(Clone, Copy, Debug)]
pub enum ShapeType {
    //primitive shapes
    Point,
    Ray,
    Sphere,
    Plane,
    Trig,
    Box,
    Rect,
    TriPrism, //5 facets, 2 triangles, 3 rectangles
    Line,
    //todo
    Frustum,
    Complex, //custom shapes
}

pub trait Shape: Vicinity<f64> {
    fn get_shape_data(&self) -> Vec<f64>;
    fn get_type(&self) -> ShapeType;
    fn get_bound(&self) -> &dyn Bound;
    //optionally returns a location of intersection of bounding shapes, preferrably closest of such locations
    fn get_intersect(&self, other: &dyn Shape) -> (bool, Option<Matrix1D>);
    //required for gjk intersection test
    fn get_support(&self, v: &Matrix1D) -> Option<Matrix1D>;
}
