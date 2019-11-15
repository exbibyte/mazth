use i_comparable::IComparableError;
use i_shape::IShape;

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
                assert!(loc.is_equal(&a._ori, 0.0001f64).unwrap());
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
