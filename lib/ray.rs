use ndarray::prelude::*;

use bound::Bound;
use shape::{Shape, ShapeType};
use vicinity::Vicinity;

use bound_aabb::AxisAlignedBBox;
use mat::*;

#[derive(Debug, Clone)]
pub struct Ray3 {
    pub _ori: Matrix1D,
    pub _dir: Matrix1D,
    pub _bound: AxisAlignedBBox,
    pub _vicinity: f64,
}

impl Ray3 {
    pub fn init(origin: &[f64], dir: &[f64]) -> Ray3 {
        assert!(origin.len() == 3);
        assert!(dir.len() == 3);
        Ray3 {
            _ori: Matrix1D::from(arr1(&[origin[0], origin[1], origin[2]])),
            _dir: Matrix1D::from(arr1(&[dir[0], dir[1], dir[2]])).normalize_l2(),
            _bound: AxisAlignedBBox::new(ShapeType::Ray, &[&origin[0..3], &dir[0..3]].concat()),
            _vicinity: 0.000001f64,
        }
    }
}

impl Shape for Ray3 {
    fn get_shape_data(&self) -> Vec<f64> {
        vec![
            self._ori[0],
            self._ori[1],
            self._ori[2],
            self._dir[0],
            self._dir[1],
            self._dir[2],
        ]
    }
    fn get_type(&self) -> ShapeType {
        ShapeType::Ray
    }
    fn get_bound(&self) -> &dyn Bound {
        &self._bound
    }
    // this shall test for intersection of bounding shapes first before procedding to test intersection using algorithms of higher complexity
    fn get_intersect(&self, other: &dyn Shape) -> (bool, Option<Matrix1D>) {
        if !self.get_bound().intersect(other.get_bound()) {
            return (false, None);
        } else {
            match other.get_type() {
                ShapeType::Ray => {
                    let other_shape_data = other.get_shape_data();
                    let ref a_dir = self._dir;
                    let b_dir = Matrix1D::from(arr1(&[
                        other_shape_data[3],
                        other_shape_data[4],
                        other_shape_data[5],
                    ]));
                    let ref a_off = self._ori;
                    let ref b_off = Matrix1D::from(arr1(&[
                        other_shape_data[0],
                        other_shape_data[1],
                        other_shape_data[2],
                    ]));

                    let c = &b_dir - a_dir;
                    let v = a_dir.cross_vec_1d(&b_dir);

                    let dot_v_c = v.inner(&c);
                    if !self.within_vicinity(dot_v_c, 0f64) {
                        //they are not in the same place, so no intersection occurs
                        return (false, None);
                    }
                    //test for colinearity
                    let d = b_off - a_off;
                    if v.t().dot(&v.view()) < 0.000_000_1f64 {
                        //lines are parallel
                        //check triangle area formed by points on ray a and b
                        let point1 = a_dir;
                        let point2 = b_off - a_off;
                        let triangle_area = point1.cross_vec_1d(&point2).norm_l2();
                        // println!( "triangle area: {}", triangle_area );
                        if !self.within_vicinity(triangle_area, 0f64) {
                            //no overlap
                            // println!( "parallel but non-overlapping lines" );
                            return (false, None);
                        } else {
                            //lines are colinear
                            let direction = if d.inner(a_dir) < 0f64 { -1f64 } else { 1f64 };
                            let distance = direction * d.norm_l2() / a_dir.norm_l2();
                            // println!( "colinear lines, distance: {}", distance );
                            if distance < 0f64 {
                                //intersection at offset of ray a, so clamp t to 0
                                return (true, Some(a_off.clone()));
                            } else {
                                //intersection at offset of ray b
                                return (true, Some(&self._dir * distance + self._ori.clone()));
                            }
                        }
                    } else {
                        //solvable intersection exists
                        let numerator = d.cross_vec_1d(&b_dir);
                        let t = numerator.norm_l2() / v.norm_l2();
                        if t < 0f64 {
                            return (false, None);
                        } else {
                            return (true, Some(&self._dir * t + self._ori.clone()));
                        }
                    }
                }
                ShapeType::Point => {
                    let other_shape_data = other.get_shape_data();
                    let b_off = Matrix1D::from(arr1(&[
                        other_shape_data[0],
                        other_shape_data[1],
                        other_shape_data[2],
                    ]));
                    let a_dir = &self._dir;
                    let a_off = &self._ori;
                    //a_dir * t + a_off = b_off
                    //t = (b_off - a_off) / a_dir
                    let t = &(&b_off - a_off) / a_dir;
                    if !self.within_vicinity(t[0], t[1]) || !self.within_vicinity(t[1], t[2]) {
                        return (false, None);
                    } else {
                        if t[0] >= 0f64 {
                            return (true, Some(&(a_dir * t[0]) + a_off));
                        } else {
                            //the point is behind the ray origin and direction
                            return (false, None);
                        }
                    }
                }
                ShapeType::Sphere => {
                    let other_shape_data = other.get_shape_data();
                    let ref b_off = Matrix1D::from(arr1(&[
                        other_shape_data[0],
                        other_shape_data[1],
                        other_shape_data[2],
                    ]));
                    let b_r = other_shape_data[3];

                    let ref a_dir = self._dir;
                    let ref a_off = self._ori;

                    //sub in the ray equation into sphere equation
                    // b := projection of relative offset onto ray direction
                    // c := (minimal possible distance between sphere and ray origin )^2
                    let relative_offset = a_off - b_off;
                    let b = relative_offset.inner(a_dir);
                    let c = relative_offset.inner(&relative_offset) - b_r * b_r;

                    if b > 0f64 && c > 0f64 {
                        //ray is outside of the sphere and points away from sphere
                        //thus no intersection occurs
                        return (false, None);
                    }

                    let d = b * b - c;
                    if d < 0f64 {
                        //ray misses sphere
                        return (false, None);
                    }

                    let t1 = -b - d.sqrt();
                    let t2 = -b + d.sqrt();

                    let t = if t1 < 0f64 {
                        t2
                    } else if t2 < 0f64 {
                        t1
                    } else if t1 < t2 {
                        t1
                    } else {
                        t2
                    };

                    return (true, Some(&(a_dir * t) + a_off));
                }
                ShapeType::Plane => {
                    let other_shape_data = other.get_shape_data();
                    let b_off = Matrix1D::from(arr1(&[
                        other_shape_data[0],
                        other_shape_data[1],
                        other_shape_data[2],
                    ]));
                    let b_nor = Matrix1D::from(arr1(&[
                        other_shape_data[3],
                        other_shape_data[4],
                        other_shape_data[5],
                    ]));
                    //ray equation: r(t) = r.offset + r.dir * t
                    //plane: p(x) = dot(normal, x-p.offset) = 0
                    //p(x) = -dot(p.normal, p.offset) + dot(p.normal, x) = 0
                    //substitution:
                    // p(t) = -dot(p.fofset,p.normal) + dot(p.normal, r.offset + r.dir*t) = 0
                    //      = -dot(p.fofset,p.normal) + dot(p.normal, r.offset) + t*dot(p.normal, r.dir) = 0
                    //t = ( dot(p.offset, p.normal) - dot(p.normal, r.offset) )/ dot(p.normal, r.dir )
                    let constant = b_off.inner(&b_nor);
                    let numerator = constant - b_nor.inner(&self._ori);
                    let denominator = b_nor.inner(&self._dir);
                    if denominator == 0f64 {
                        //ray direction is colplaner to the plane
                        if constant == self._ori.inner(&b_nor) {
                            return (true, Some(self._ori.clone()));
                        } else {
                            return (false, None);
                        }
                    } else if denominator > 0f64 {
                        //ray direction is not facing plane normal
                        return (false, None);
                    }
                    let t = numerator / denominator;
                    if t < 0f64 {
                        return (false, None);
                    }
                    return (true, Some(&(&self._dir * t) + &self._ori));
                }
                _ => {
                    unimplemented!();
                }
            }
        }
    }
    fn get_support(&self, _v: &Matrix1D) -> Option<Matrix1D> {
        None
    }
}

impl Vicinity<f64> for Ray3 {
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
