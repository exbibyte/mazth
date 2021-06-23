use test::util::*;

#[test]
fn test_mat3x3() {
    use mat::Mat3x1;
    use mat::Mat3x3;
    {
        let v0 = Mat3x3::new_r([10f64; 9]);
        let v1 = Mat3x3::new_r([10f64; 9]);
        let v2 = &v0 + &v1;
        assert!(v2.equal(&Mat3x3::new_r([20f64; 9])));
    }
    {
        let v0 = Mat3x3::new_r([10f64; 9]);
        let v1 = Mat3x3::new_r([10f64; 9]);
        let v2 = &v0 - &v1;
        assert!(v2.equal(&Mat3x3::new_r([0f64; 9])));
    }
    {
        let v0 = Mat3x3::new_r([20f64; 9]);
        let v1 = Mat3x3::new_r([2f64; 9]);
        let v2 = &v0 / &v1;
        assert!(v2.equal(&Mat3x3::new_r([10f64; 9])));
    }
    {
        let v0 = Mat3x3::new_r([10f64; 9]);
        let v1 = &v0 * 3f64;
        assert!(v1.equal(&Mat3x3::new_r([30f64; 9])));
    }
    {
        let a0 = (1..10).map(|x| x as f64).collect::<Vec<_>>();
        let a1 = (10..19).map(|x| x as f64).collect::<Vec<_>>();

        let v0 = Mat3x3::new_r(from_slice_9(a0.as_slice()));
        let v1 = Mat3x3::new_r(from_slice_9(a1.as_slice()));

        let v2 = v0.dot(&v1);
        let v3 = Mat3x3::new_r([84., 90., 96., 201., 216., 231., 318., 342., 366.]);

        assert!(v2.equal(&v3));
    }
    {
        //3x3 mul 3x1
        let v0 = Mat3x1::new([1f64, 2f64, 3f64]);
        let a0 = (1..10).map(|x| x as f64).collect::<Vec<_>>();
        let mut v1 = Mat3x3::new_r(from_slice_9(a0.as_slice()));
        let v2 = v1.dot_vec(&v0);
        assert!(v2.equal(&Mat3x1::new([14f64, 32f64, 50f64])));
    }
    {
        //transpose
        let a0 = (0..9).map(|x| x as f64).collect::<Vec<_>>();
        let v0 = Mat3x3::new_r(from_slice_9(a0.as_slice()));
        let v1 = v0.t();
        let v2 = Mat3x3::new_r([0., 3., 6., 1., 4., 7., 2., 5., 8.]);
        assert!(v1.equal(&v2));
    }
    {
        //identity
        let v1 = Mat3x3::eye();
        assert!(v1.equal(&Mat3x3::new_r([
            1f64, 0f64, 0f64, 0f64, 1f64, 0f64, 0f64, 0f64, 1f64,
        ])));
    }
    {
        //inverse
        let a = Mat3x3::new_r([3., 3., 3., 0., 6., 0., 0., 0., 9.]);
        let b = a.inv().expect("inverse");
        assert!(b.equal(&Mat3x3::new_r([
            1. / 3.,
            -1. / 6.,
            -1. / 9.,
            0.,
            1. / 6.,
            0.,
            0.,
            0.,
            1. / 9.
        ])));
    }
}
