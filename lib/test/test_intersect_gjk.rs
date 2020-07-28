use intersect_gjk;
use mat::*;
use ndarray::prelude::*;
use shape::Shape;
use sphere::Sphere;

#[test]
fn test_intersect_gjk_shape_support() {
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                let a = Sphere::init(&[-5f64, 2.5f64, 15f64], 5.5f64);
                let v_x = 0.2f64 * (i as f64);
                let v_y = 0.2f64 * (j as f64);
                let v_z = 0.2f64 * (k as f64);

                let v = Matrix1D::from(arr1(&[v_x, v_y, v_z]));

                match a.get_support(&v) {
                    Some(o) => {
                        let l = (v_x * v_x + v_y * v_y + v_z * v_z).sqrt();
                        assert!(Mat3x1::from(o).equal(&Mat3x1::new([
                            -5f64 + v_x / l * 5.5f64,
                            2.5f64 + v_y / l * 5.5f64,
                            15f64 + v_z / l * 5.5f64
                        ])));
                    }
                    _ => {
                        if i != 0 || j != 0 || k != 0 {
                            panic!("unexpected result");
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_intersect_gjk_query_intersect_positive() {
    {
        let a = Sphere::init(&[0f64, 0f64, 0f64], 5f64);
        let b = Sphere::init(&[7f64, 0f64, 0f64], 2.1f64);
        let ret = intersect_gjk::query_intersect(&a, &b);
        assert!(ret.expect("gjk return unexpected"));
    }
    {
        let a = Sphere::init(&[0f64, 5f64, 0f64], 5f64);
        let b = Sphere::init(&[0f64, 0f64, 0f64], 2f64);
        let ret = intersect_gjk::query_intersect(&a, &b);
        assert!(ret.expect("gjk return unexpected"));
    }
    {
        let a = Sphere::init(&[0f64, 5f64, 0f64], 10f64);
        let b = Sphere::init(&[1f64, 1f64, 0f64], 2f64);
        let ret = intersect_gjk::query_intersect(&a, &b);
        assert!(ret.expect("gjk return unexpected"));
    }
    {
        let a = Sphere::init(&[0f64, -5f64, 0f64], 5f64);
        let b = Sphere::init(&[0f64, 4.999f64, 0f64], 5f64);
        let ret = intersect_gjk::query_intersect(&a, &b);
        assert!(ret.expect("gjk return unexpected"));
    }
    //todo
    // {
    //     let a = Point3::init( &[ -9.9, 9.9, 9.9 ] );
    //     let b = RecBox::init( &[ 0.,0.,0. ], 10. );
    //     let ret = intersect_gjk::query_intersect( &a, &b );
    //     assert!( ret.expect("gjk return unexpected") );
    // }
}

#[test]
fn test_intersect_gjk_query_intersect_negative() {
    {
        let a = Sphere::init(&[0f64, 0f64, 0f64], 5f64);
        let b = Sphere::init(&[7f64, 0f64, 0f64], 1.99f64);
        let ret = intersect_gjk::query_intersect(&a, &b);
        assert!(!ret.expect("gjk return unexpected"));
    }
    {
        let a = Sphere::init(&[0f64, 5f64, 0f64], 5f64);
        let b = Sphere::init(&[0f64, 0f64, 10f64], 2f64);
        let ret = intersect_gjk::query_intersect(&a, &b);
        assert!(!ret.expect("gjk return unexpected"));
    }
}
