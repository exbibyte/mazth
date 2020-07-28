#[test]
fn test_mat3x1() {
    use mat::{Mat1x3, Mat3x1};

    {
        let v0 = Mat3x1::new([10f64; 3]);
        let v1 = Mat3x1::new([2f64; 3]);
        let v2 = &v0 * &v1;
        assert!(v2.equal(&Mat3x1::new([20f64; 3])));
    }
    {
        let v0 = Mat3x1::new([10f64; 3]);
        let v1 = Mat3x1::new([2f64; 3]);
        let v2 = &v0 + &v1;
        assert!(v2.equal(&Mat3x1::new([12f64; 3])));
    }
    {
        let v0 = Mat3x1::new([10f64; 3]);
        let v1 = Mat3x1::new([2f64; 3]);
        let v2 = &v0 - &v1;
        assert!(v2.equal(&Mat3x1::new([8f64; 3])));
    }
    {
        let v0 = Mat3x1::new([2f64; 3]);
        assert!((v0.norm_l2() - (2f64 * 2f64 * 3f64).sqrt()).abs() < 1e-9);
    }
    {
        let v0 = Mat3x1::new([2f64; 3]);
        let v1 = v0.normalize_l2();
        assert!(v1.equal(&Mat3x1::new([2f64 / (2f64 * 2f64 * 3f64).sqrt(); 3])));
    }
    {
        let v0 = Mat3x1::new([2f64; 3]);
        let v1 = &v0 * 4.;
        assert!(v1.equal(&Mat3x1::new([8.; 3])));
    }
    {
        let v0 = Mat3x1::new([2., 3., 4.]);
        let v1 = Mat3x1::new([5., 6., 7.]);
        let v2 = v0.cross(&v1);
        assert!(v2.equal(&Mat3x1::new([-3., 6., -3.])));
    }
    {
        let v0 = Mat3x1::new([2., 3., 4.]);
        let v1 = v0.t();
        assert!(v1.equal(&Mat1x3::new([2., 3., 4.])));
    }
    {
        let v0 = Mat3x1::new([2., 3., 4.]);
        let v1 = Mat3x1::new([5., 6., 7.]);
        let v2 = v0.inner(&v1);
        assert!((v2 - (10. + 18. + 28.)).abs() < 1e-9);
    }
}
