use shape::Shape;

use mat::*;
use point::Point3;
use rbox::RecBox;

#[test]
fn test_intersect_point_point() {
    //intersection
    {
        let a = Point3::init(&[-9.9, 9.9, 9.9]);
        let b = RecBox::init(&[0., 0., 0.], 10.);
        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::from(a._ori)));
            }
            _ => panic!("unexpected result for ray point intersection"),
        }
    }
    //no intersection
    {
        let a = Point3::init(&[-9.9, 9.9, -10.1]);
        let b = RecBox::init(&[0., 0., 0.], 10.);
        match a.get_intersect(&b) {
            (false, None) => (),
            _ => panic!("unexpected result for ray point intersection"),
        }
    }
}
