use mat::*;
use plane::Plane;
use ray::Ray3;
use shape::Shape;

#[test]
fn test_intersect_ray_plane() {
    //Ray Plane intersection
    {
        let a = Ray3::init(&[5f64, 5f64, 5f64], &[-1f64, -1f64, -1f64]);
        let b = Plane::init(&[1f64, 1f64, 1f64], &[0f64, 1f64, 0f64]);
        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::new([1f64, 1f64, 1f64])));
            }
            _ => panic!("unexpected result for ray sphere intersection"),
        }
    }

    //Ray Plane no intersection, plane normal and ray dir same direction
    {
        let a = Ray3::init(&[0f64, 0f64, 0f64], &[1f64, 1f64, 1f64]);
        let b = Plane::init(&[1f64, 1f64, 1f64], &[0f64, 1f64, 0f64]);
        match a.get_intersect(&b) {
            (false, None) => (),
            _ => panic!("unexpected result for ray sphere no intersection"),
        }
    }

    //Ray Plane no intersection, coplanar
    {
        let a = Ray3::init(&[5f64, 5f64, 5f64], &[1f64, 0f64, 1f64]);
        let b = Plane::init(&[1f64, 1f64, 1f64], &[0f64, 1f64, 0f64]);
        match a.get_intersect(&b) {
            (false, None) => (),
            _ => panic!("unexpected result for ray sphere no intersection"),
        }
    }

    //Ray Plane intersection, coplanar
    {
        let a = Ray3::init(&[1f64, 1f64, 1f64], &[1f64, 0f64, 1f64]);
        let b = Plane::init(&[1f64, 1f64, 1f64], &[0f64, 1f64, 0f64]);
        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::from(a._ori)));
            }
            _ => panic!("unexpected result for ray sphere intersection"),
        }
    }
}
