use mat::*;
use ray::Ray3;
use shape::Shape;
use sphere::Sphere;

#[test]
fn test_intersect_ray_sphere() {
    //Ray Sphere intersection
    {
        let a = Ray3::init(&[5f64, 0f64, 0f64], &[1f64, 0f64, 0f64]);
        let b = Sphere::init(&[20f64, 0f64, 0f64], 5f64);
        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                dbg!(&loc);
                assert!(Mat3x1::from(loc).equal(&Mat3x1::new([15f64, 0f64, 0f64])));
            }
            _ => panic!("unexpected result for ray sphere intersection"),
        }
    }

    //Ray Sphere no intersection, opposing direction
    {
        let a = Ray3::init(&[5f64, 0f64, 0f64], &[-1f64, 0f64, 0f64]);
        let b = Sphere::init(&[20f64, 0f64, 0f64], 5f64);
        match a.get_intersect(&b) {
            (false, None) => (),
            _ => panic!("unexpected result for ray sphere no intersection"),
        }
    }

    //Ray Sphere intersection, at edge
    {
        let a = Ray3::init(&[30f64, 10f64, 10f64], &[-1f64, 0f64, 0f64]);
        let b = Sphere::init(&[20f64, 10f64, 10f64], 5f64);
        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::new([25f64, 10f64, 10f64])));
            }
            _ => panic!("unexpected result for ray sphere intersection"),
        }
    }

    //Ray Sphere intersection, oblique angle
    {
        let a = Ray3::init(&[30f64, 10f64, 10f64], &[-1f64, -1f64, -1f64]);
        let b = Sphere::init(&[20f64, 0f64, 0f64], 5f64);
        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                let c = (25f64 / 3f64).sqrt();
                println!("loc: {:?}", loc);
                assert!(Mat3x1::from(loc).equal(&Mat3x1::new([20f64 + c, 0f64 + c, 0f64 + c])));
            }
            _ => panic!("unexpected result for ray sphere intersection"),
        }
    }

    //Ray Sphere no intersection
    {
        let a = Ray3::init(&[30f64, 10f64, 10f64], &[-1f64, 0f64, -1f64]);
        let b = Sphere::init(&[20f64, 0f64, 0f64], 5f64);
        match a.get_intersect(&b) {
            (false, None) => (),
            _ => panic!("unexpected result for ray sphere no intersection"),
        }
    }
}
