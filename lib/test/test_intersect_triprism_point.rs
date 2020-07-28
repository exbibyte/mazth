use mat::*;
use point::Point3;
use shape::Shape;
use triprism::TriPrism;

#[test]
fn test_intersect_point() {
    //intersection
    {
        let a = TriPrism::init(&[0., 0., 0., 1., 0., 0., 1., 1., 0.], 1.);
        let b = Point3::init(&[0.25, 0.25, 0.]);
        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::from(b._ori)));
            }
            _ => panic!("unexpected result for triprism point intersection"),
        }
    }
    //intersection
    {
        let a = TriPrism::init(&[0., 0., 0., 1., 0., 0., 1., 1., 0.], 1.);
        let b = Point3::init(&[0.25, 0.25, 0.5]);
        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::from(b._ori)));
            }
            _ => panic!("unexpected result for triprism point intersection"),
        }
    }
    //intersection
    {
        let a = TriPrism::init(&[0., 0., 0., 1., 0., 0., 1., 1., 0.], 1.);
        let b = Point3::init(&[0.25, 0.25, 1.]);
        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::from(b._ori)));
            }
            _ => panic!("unexpected result for triprism point intersection"),
        }
    }

    //intersection, flipped
    {
        let a = TriPrism::init(&[0., 0., 0., 1., 0., 0., 1., 1., 0.], 1.);
        let b = Point3::init(&[0.25, 0.25, 0.]);
        match b.get_intersect(&a) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::from(b._ori)));
            }
            _ => panic!("unexpected result for triprism point intersection"),
        }
    }
    //intersection, flipped
    {
        let a = TriPrism::init(&[0., 0., 0., 1., 0., 0., 1., 1., 0.], 1.);
        let b = Point3::init(&[0.25, 0.25, 0.5]);
        match b.get_intersect(&a) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::from(b._ori)));
            }
            _ => panic!("unexpected result for triprism point intersection"),
        }
    }
    //intersection, flipped
    {
        let a = TriPrism::init(&[0., 0., 0., 1., 0., 0., 1., 1., 0.], 1.);
        let b = Point3::init(&[0.25, 0.25, 1.]);
        match b.get_intersect(&a) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::from(b._ori)));
            }
            _ => panic!("unexpected result for triprism point intersection"),
        }
    }

    //no intersection
    {
        let a = TriPrism::init(&[0., 0., 0., 1., 0., 0., 1., 1., 0.], 1.);
        let b = Point3::init(&[0.25, 0.25, 1.001]);
        match a.get_intersect(&b) {
            (false, None) => (),
            _ => panic!("unexpected result for triprism point no intersection"),
        }
    }
    //no intersection
    {
        let a = TriPrism::init(&[0., 0., 0., 1., 0., 0., 1., 1., 0.], 1.);
        let b = Point3::init(&[0.5, 0.55, 0.5]);
        match a.get_intersect(&b) {
            (false, None) => (),
            _ => panic!("unexpected result for triprism point no intersection"),
        }
    }
}
