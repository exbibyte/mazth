use mat::*;
use plane::Plane;
use shape::Shape;
use sphere::Sphere;

#[test]
fn test_intersect_sphere_plane() {
    //sphere plane intersection
    {
        let a = Sphere::init(&[10f64, 0f64, 0f64], 5f64);
        let b = Plane::init(&[5f64, 0f64, 0f64], &[1f64, 0f64, 0f64]);
        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::new([5f64, 0f64, 0f64])));
            }
            _ => panic!("unexpected result for sphere plane intersection"),
        }
    }

    //sphere plane intersection
    {
        let a = Sphere::init(&[8f64, 4f64, 6f64], 5f64);
        let b = Plane::init(&[5f64, 0f64, 0f64], &[1f64, 0f64, 0f64]);
        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::new([5f64, 4f64, 6f64])));
            }
            _ => panic!("unexpected result for sphere plane intersection"),
        }
    }

    //sphere plane intersection
    {
        let a = Sphere::init(&[10f64, 4f64, 6f64], 5f64);
        let b = Plane::init(&[10f64, 4f64, 6f64], &[1f64, 1f64, 1f64]);
        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::new([10f64, 4f64, 6f64])));
            }
            _ => panic!("unexpected result for sphere plane intersection"),
        }
    }

    //sphere plane no intersection
    {
        let a = Sphere::init(&[10.1f64, 4f64, 6f64], 5f64);
        let b = Plane::init(&[5f64, 0f64, 0f64], &[1f64, 0f64, 0f64]);
        match a.get_intersect(&b) {
            (false, None) => (),
            _ => panic!("unexpected result for sphere plane no intersection"),
        }
    }
}
