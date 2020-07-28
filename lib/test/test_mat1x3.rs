use test::util::*;

#[test]
fn test_mat1x3() {
    use mat::{Mat1x3, Mat3x1, Mat3x3};
    {
        let v0 = Mat1x3::new([10f64; 3]);
        let v1 = Mat1x3::new([2f64; 3]);
        let v2 = &v0 * &v1;
        assert!(v2.equal(&Mat1x3::new([20f64; 3])));
    }
    {
        let v0 = Mat1x3::new([10f64; 3]);
        let v1 = Mat1x3::new([2f64; 3]);
        let v2 = &v0 + &v1;
        assert!(v2.equal(&Mat1x3::new([12f64; 3])));
    }
    {
        let v0 = Mat1x3::new([10f64; 3]);
        let v1 = Mat1x3::new([2f64; 3]);
        let v2 = &v0 - &v1;
        assert!(v2.equal(&Mat1x3::new([8f64; 3])));
    }
    {
        let v0 = Mat1x3::new([2f64; 3]);
        assert!((v0.norm_l2() - (2f64 * 2f64 * 3f64).sqrt()).abs() < 1e-9);
    }
    {
        let v0 = Mat1x3::new([2f64; 3]);
        let v1 = v0.normalize_l2();
        assert!(v1.equal(&Mat1x3::new([2f64 / (2f64 * 2f64 * 3f64).sqrt(); 3])));
    }
    {
        let v0 = Mat1x3::new([2f64; 3]);
        let v1 = &v0 * 4.;
        assert!(v1.equal(&Mat1x3::new([8.; 3])));
    }
    {
        let v0 = Mat1x3::new([2., 3., 4.]);
        let v1 = Mat1x3::new([5., 6., 7.]);
        let v2 = v0.cross(&v1);
        assert!(v2.equal(&Mat1x3::new([-3., 6., -3.])));
    }
    {
        let v0 = Mat1x3::new([2., 3., 4.]);
        let v1 = v0.t();
        assert!(v1.equal(&Mat3x1::new([2., 3., 4.])));
    }
    {
        //mat1x3 mul mat3x3
        let v0 = Mat1x3::new([1f64, 2f64, 3f64]);
        let v1 = Mat3x3::new_r([0., 1., 2., 3., 4., 5., 6., 7., 8.]);
        let v2 = v0.dot(&v1);
        assert!(v2.equal(&Mat1x3::new([24., 30., 36.])));
    }
    {
        let v0 = Mat1x3::new([2., 3., 4.]);
        let v1 = Mat1x3::new([5., 6., 7.]);
        let v2 = v0.inner(&v1);
        assert!((v2 - (10. + 18. + 28.)).abs() < 1e-9);
    }
    {
        //mat1x3 mul mat3x1
        let v0 = Mat1x3::new([2., 3., 4.]);
        let v1 = Mat3x1::new([5., 6., 7.]);
        let v2 = v0.dot_vec(&v1);
        assert!((v2 - (10. + 18. + 28.)).abs() < 1e-9);
    }
}
