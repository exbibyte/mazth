use mat::*;
use point::Point3;
use shape::Shape;

#[test]
fn test_intersect_point_point() {
    //point point intersection
    {
        let a = Point3::init(&[25f64, 5f64, 5f64]);
        let b = Point3::init(&[25f64, 5f64, 5f64]);
        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::from(b._ori)));
            }
            _ => panic!("unexpected result for ray point intersection"),
        }
    }
    //point point no intersection
    {
        let a = Point3::init(&[25f64, 5f64, 5f64]);
        let b = Point3::init(&[25.1f64, 5f64, 5f64]);
        match a.get_intersect(&b) {
            (false, None) => (),
            _ => panic!("unexpected result for ray point intersection"),
        }
    }
}
