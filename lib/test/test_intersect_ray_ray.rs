use mat::*;
use ray::Ray3;
use shape::Shape;

#[test]
fn test_intersect_ray_ray() {
    //parallel rays, no intersection
    {
        let a = Ray3::init(&[20f64, 0f64, 0f64], &[1f64, 1f64, 1f64]);
        let b = Ray3::init(&[25f64, 0f64, 0f64], &[1f64, 1f64, 1f64]);

        match a.get_intersect(&b) {
            (false, None) => (),
            _ => panic!("unexpected result for parallel rays, no intersection"),
        }
    }

    //colinear rays, intersection
    {
        let a = Ray3::init(&[20f64, 0f64, 0f64], &[1f64, 1f64, 1f64]);
        let b = Ray3::init(&[22f64, 2f64, 2f64], &[1f64, 1f64, 1f64]);

        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::from(b._ori)));
            }
            _ => panic!("unexpected result for parallel rays, no intersection"),
        }
    }

    //colinear rays, intersection
    {
        let a = Ray3::init(&[25f64, 5f64, 5f64], &[1f64, 1f64, 1f64]);
        let b = Ray3::init(&[22f64, 2f64, 2f64], &[1f64, 1f64, 1f64]);

        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::from(a._ori)));
            }
            _ => panic!("unexpected result for parallel rays, no intersection"),
        }
    }

    //rays, intersection
    {
        let a = Ray3::init(&[5f64, 5f64, 0f64], &[-1f64, 0f64, 0f64]);
        let b = Ray3::init(&[0f64, 0f64, 0f64], &[0f64, 1f64, 0f64]);

        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::new([0f64, 5f64, 0f64])));
            }
            _ => panic!("unexpected result for ray intersection"),
        }
    }

    //non-coplaner rays, no intersection
    {
        let a = Ray3::init(&[5f64, 5f64, 2f64], &[-1f64, -1f64, 0f64]);
        let b = Ray3::init(&[5f64, 5f64, 0f64], &[1f64, 1f64, 0f64]);

        match a.get_intersect(&b) {
            (false, None) => (),
            _ => panic!("unexpected result for ray intersection"),
        }
    }
}
