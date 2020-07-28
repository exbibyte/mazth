#[test]
fn test_mat1x4() {
    use mat::{Mat1x4, Mat4x1, Mat4x4};
    {
        let v0 = Mat1x4::new([10f64; 4]);
        let v1 = Mat1x4::new([2f64; 4]);
        let v2 = &v0 * &v1;
        assert!(v2.equal(&Mat1x4::new([20f64; 4])));
    }
    {
        let v0 = Mat1x4::new([10f64; 4]);
        let v1 = Mat1x4::new([2f64; 4]);
        let v2 = &v0 + &v1;
        assert!(v2.equal(&Mat1x4::new([12f64; 4])));
    }
    {
        let v0 = Mat1x4::new([10f64; 4]);
        let v1 = Mat1x4::new([2f64; 4]);
        let v2 = &v0 - &v1;
        assert!(v2.equal(&Mat1x4::new([8f64; 4])));
    }
    {
        let v0 = Mat1x4::new([2f64; 4]);
        assert!((v0.norm_l2() - (2f64 * 2f64 * 4f64).sqrt()).abs() < 1e-9);
    }
    {
        let v0 = Mat1x4::new([2f64; 4]);
        let v1 = v0.normalize();
        assert!(v1.equal(&Mat1x4::new([2f64 / (2f64 * 2f64 * 4f64).sqrt(); 4])));
    }
    {
        let v0 = Mat1x4::new([2f64; 4]);
        let v1 = &v0 * 4.;
        assert!(v1.equal(&Mat1x4::new([8.; 4])));
    }
    {
        let v0 = Mat1x4::new([1., 2., 3., 4.]);
        let v1 = v0.t();
        assert!(v1.equal(&Mat4x1::new([1., 2., 3., 4.])));
    }
    {
        let v0 = Mat1x4::new([1., 2., 3., 4.]);
        let v1 = Mat1x4::new([5., 6., 7., 8.]);
        let v2 = v0.inner(&v1);
        assert!((v2 - (5. + 12. + 21. + 32.)).abs() < 1e-9);
    }
    {
        //mat1x4 mul mat4x1
        let v0 = Mat1x4::new([1., 2., 3., 4.]);
        let v1 = Mat4x1::new([5., 6., 7., 8.]);
        let v2 = v0.dot_vec(&v1);
        assert!((v2 - (5. + 12. + 21. + 32.)).abs() < 1e-9);
    }
    {
        //mat1x4 mul mat4x4
        let v0 = Mat1x4::new([1f64, 2f64, 3f64, 4f64]);
        let v1 = Mat4x4::new_r([
            0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15.,
        ]);
        let v2 = v0.dot(&v1);
        assert!(v2.equal(&Mat1x4::new([80., 90., 100., 110.])));
    }
}
