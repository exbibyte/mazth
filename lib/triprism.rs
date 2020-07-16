use ndarray::prelude::*;

use bound::IBound;
///triangular prism (5 faces: 2 triangles, 3 squares)
use shape::{IShape, ShapeType};
use vicinity::IVicinity;

use bound_aabb::AxisAlignedBBox;
use mat::*;

use plane::Plane;
use ray::Ray3;

#[derive(Debug, Clone, Default)]
pub struct TriPrism {
    ///base
    pub _tri_base: [Matrix1D; 3],

    ///base + height offset in normal direction of base
    pub _tri_base2: [Matrix1D; 3],

    ///normal of the triangle base, scaled with height
    pub _normal_height: Matrix1D,

    pub _bound: AxisAlignedBBox,

    pub _vicinity: f64,
}

impl TriPrism {
    /// initialize with tribase: base vertices in ccw order
    pub fn init(tri_base: &[f64], height: f64) -> TriPrism {
        assert!(tri_base.len() == 9);

        let v0 = Matrix1D::from(arr1(&[tri_base[0], tri_base[1], tri_base[2]]));
        let v1 = Matrix1D::from(arr1(&[tri_base[3], tri_base[4], tri_base[5]]));
        let v2 = Matrix1D::from(arr1(&[tri_base[6], tri_base[7], tri_base[8]]));

        let d1 = &v1 - &v0;
        let d2 = &v2 - &v0;
        let normal = d1.cross_vec_1d(&d2).normalize_l2();
        let h_offset = normal * height;

        let v00 = &v0 + &h_offset;
        let v11 = &v1 + &h_offset;
        let v22 = &v2 + &h_offset;

        let base = [v0, v1, v2];
        let base2 = [v00, v11, v22];

        use std::cmp::Ordering::*;

        let xs = [
            base[0][0],
            base[1][0],
            base[2][0],
            base2[0][0],
            base2[1][0],
            base2[2][0],
        ];

        let ys = [
            base[0][1],
            base[1][1],
            base[2][1],
            base2[0][1],
            base2[1][1],
            base2[2][1],
        ];

        let zs = [
            base[0][2],
            base[1][2],
            base[2][2],
            base2[0][2],
            base2[1][2],
            base2[2][2],
        ];

        let x_min = *xs
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Equal))
            .unwrap();
        let x_max = *xs
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Equal))
            .unwrap();

        let y_min = *ys
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Equal))
            .unwrap();
        let y_max = *ys
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Equal))
            .unwrap();

        let z_min = *zs
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Equal))
            .unwrap();
        let z_max = *zs
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Equal))
            .unwrap();

        TriPrism {
            _tri_base: base,
            _tri_base2: base2,
            _normal_height: h_offset,
            _bound: AxisAlignedBBox::init(
                ShapeType::Rect,
                &[x_min, y_min, z_min, x_max, y_max, z_max],
            ),
            _vicinity: 0.000001f64,
        }
    }
}

impl IShape for TriPrism {
    fn get_shape_data(&self) -> Vec<f64> {
        vec![
            self._tri_base[0][0],
            self._tri_base[0][1],
            self._tri_base[0][2],
            self._tri_base[1][0],
            self._tri_base[1][1],
            self._tri_base[1][2],
            self._tri_base[2][0],
            self._tri_base[2][1],
            self._tri_base[2][2],
            self._normal_height[0],
            self._normal_height[1],
            self._normal_height[2],
        ]
    }
    fn get_type(&self) -> ShapeType {
        ShapeType::TriPrism
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
                    let other_shape_data = other.get_shape_data();
                    let other_point = Matrix1D::from(arr1(&[
                        other_shape_data[0],
                        other_shape_data[1],
                        other_shape_data[2],
                    ]));

                    //test point aginst 5 half spaces from facets of the tri_prism to determine if point is inside the tri_prism

                    let ref n = self._normal_height;

                    let tests = vec![
                        (&self._tri_base[0], n * -1.),
                        (&self._tri_base2[0], n.clone()),
                        (
                            &self._tri_base[0],
                            (&self._tri_base[1] - &self._tri_base[0]).cross_vec_1d(&n)
                        ),
                        (
                            &self._tri_base[1],
                            (&self._tri_base[2] - &self._tri_base[1]).cross_vec_1d(&n)
                        ),
                        (
                            &self._tri_base[2],
                            (&self._tri_base[0] - &self._tri_base[2]).cross_vec_1d(&n)
                        ),
                    ];

                    let is_inside = tests
                        .iter()
                        .all(|(vert, normal)| !((&other_point - *vert).dot(normal) > 0.));

                    if is_inside {
                        (true, Some(other_point))
                    } else {
                        (false, None)
                    }
                }
                ShapeType::Line => {
                    let other_shape_data = other.get_shape_data();
                    let a = Matrix1D::from(arr1(&[
                        other_shape_data[0],
                        other_shape_data[1],
                        other_shape_data[2],
                    ]));
                    let b = Matrix1D::from(arr1(&[
                        other_shape_data[3],
                        other_shape_data[4],
                        other_shape_data[5],
                    ]));

                    //test points aginst 5 half spaces from facets of the tri_prism to determine if point is inside the tri_prism

                    let ref n = self._normal_height;

                    let tests = vec![
                        (&self._tri_base[0], n * -1.),
                        (&self._tri_base2[0], n.clone()),
                        (
                            &self._tri_base[0],
                            (&self._tri_base[1] - &self._tri_base[0]).cross_vec_1d(&n)
                        ),
                        (
                            &self._tri_base[1],
                            (&self._tri_base[2] - &self._tri_base[1]).cross_vec_1d(&n)
                        ),
                        (
                            &self._tri_base[2],
                            (&self._tri_base[0] - &self._tri_base[2]).cross_vec_1d(&n)
                        ),
                    ];

                    let a_is_inside = tests
                        .iter()
                        .all(|(vert, normal)| !((&a - *vert).dot(normal) > 0.));
                    let b_is_inside = tests
                        .iter()
                        .all(|(vert, normal)| !((&b - *vert).dot(normal) > 0.));

                    if a_is_inside {
                        return (true, Some(a));
                    } else if b_is_inside {
                        return (true, Some(b));
                    }

                    //continue test using ray plane intersection

                    let v = &b - &a;
                    let mag = v.norm_l2();

                    let r = Ray3::init(&[a[0], a[1], a[2]], &[v[0], v[1], v[2]]);

                    let ref n1 = self._normal_height;
                    let n0 = n1 * -1.;
                    let n2 = (&self._tri_base[1] - &self._tri_base[0]).cross_vec_1d(&n1);
                    let n3 = (&self._tri_base[2] - &self._tri_base[1]).cross_vec_1d(&n1);
                    let n4 = (&self._tri_base[0] - &self._tri_base[2]).cross_vec_1d(&n1);

                    let facets = vec![
                        Plane::init(
                            &[
                                self._tri_base[0][0] as f64,
                                self._tri_base[0][1] as f64,
                                self._tri_base[0][2] as f64,
                            ],
                            &[n0[0] as f64, n0[1] as f64, n0[2] as f64],
                        ),
                        Plane::init(
                            &[
                                self._tri_base2[0][0] as f64,
                                self._tri_base2[0][1] as f64,
                                self._tri_base2[0][2] as f64,
                            ],
                            &[n1[0] as f64, n1[1] as f64, n1[2] as f64],
                        ),
                        Plane::init(
                            &[
                                self._tri_base[0][0] as f64,
                                self._tri_base[0][1] as f64,
                                self._tri_base[0][2] as f64,
                            ],
                            &[n2[0] as f64, n2[1] as f64, n2[2] as f64],
                        ),
                        Plane::init(
                            &[
                                self._tri_base[1][0] as f64,
                                self._tri_base[1][1] as f64,
                                self._tri_base[1][2] as f64,
                            ],
                            &[n3[0] as f64, n3[1] as f64, n3[2] as f64],
                        ),
                        Plane::init(
                            &[
                                self._tri_base[2][0] as f64,
                                self._tri_base[2][1] as f64,
                                self._tri_base[2][2] as f64,
                            ],
                            &[n4[0] as f64, n4[1] as f64, n4[2] as f64],
                        ),
                    ];

                    let mut intersect_point = None;
                    let mut is_inside = false;
                    for i in facets.iter() {
                        let res = r.get_intersect(i);
                        if res.0 {
                            let collide_point = res.1.as_ref().unwrap();
                            let mag2 = (collide_point - &a).norm_l2();

                            //one more check necesary for the candidate collision point
                            let is_point_inside = tests
                                .iter()
                                .all(|(vert, normal)| !((collide_point - *vert).dot(normal) > 0.));

                            if !is_point_inside || mag2 > mag {
                                continue;
                            } else {
                                is_inside = true;
                                intersect_point = res.1;
                                break;
                            }
                        }
                    }

                    if is_inside {
                        (true, intersect_point)
                    } else {
                        (false, None)
                    }
                }
                _ => {
                    unimplemented!();
                }
            }
        }
    }
    fn get_support(&self, v: &Matrix1D) -> Option<Matrix1D> {
        if v.norm_l2() > 0.000_001f64 {
            //get a furthest point in the given direction v
            let points = [
                &self._tri_base[0],
                &self._tri_base[1],
                &self._tri_base[2],
                &self._tri_base2[0],
                &self._tri_base2[1],
                &self._tri_base2[2],
            ];

            let furthest = points
                .iter()
                .map(|x| x.dot(v))
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            let o = points[furthest.0].clone();

            Some(o)
        } else {
            None
        }
    }
}

impl IVicinity<f64> for TriPrism {
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
