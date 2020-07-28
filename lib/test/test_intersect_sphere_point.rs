use mat::*;
use point::Point3;
use shape::Shape;
use sphere::Sphere;

#[test]
fn test_intersect_sphere_point() {
    //sphere point intersection
    {
        let a = Sphere::init(&[10f64, 0f64, 0f64], 5f64);
        let b = Point3::init(&[8f64, 2f64, 3f64]);
        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::new([8f64, 2f64, 3f64])));
            }
            _ => panic!("unexpected result for ray point intersection"),
        }
    }
    //sphere point intersection
    {
        let a = Sphere::init(&[10f64, 0f64, 0f64], 5f64);
        let b = Point3::init(&[10f64, 5f64, 0f64]);
        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::new([10f64, 5f64, 0f64])));
            }
            _ => panic!("unexpected result for ray point intersection"),
        }
    }
    //sphere point no intersection
    {
        let a = Sphere::init(&[10f64, 0f64, 0f64], 5f64);
        let b = Point3::init(&[0f64, 5.1f64, 0f64]);
        match a.get_intersect(&b) {
            (false, None) => (),
            _ => panic!("unexpected result for ray point no intersection"),
        }
    }
}
