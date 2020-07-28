use test::util::*;

#[test]
fn test_mat4x4() {
    use mat::Mat3x3;
    use mat::Mat4x1;
    use mat::Mat4x4;
    {
        let v0 = Mat4x4::new_r([10f64; 16]);
        let v1 = Mat4x4::new_r([10f64; 16]);
        let v2 = &v0 + &v1;
        assert!(v2.equal(&Mat4x4::new_r([20f64; 16])));
    }
    {
        let v0 = Mat4x4::new_r([10f64; 16]);
        let v1 = Mat4x4::new_r([10f64; 16]);
        let v2 = &v0 - &v1;
        assert!(v2.equal(&Mat4x4::new_r([0f64; 16])));
    }
    {
        let v0 = Mat4x4::new_r([20f64; 16]);
        let v1 = Mat4x4::new_r([2f64; 16]);
        let v2 = &v0 / &v1;
        assert!(v2.equal(&Mat4x4::new_r([10f64; 16])));
    }
    {
        let v0 = Mat4x4::new_r([10f64; 16]);
        let v1 = &v0 * 3f64;
        assert!(v1.equal(&Mat4x4::new_r([30f64; 16])));
    }
    {
        let a0 = (1..17).map(|x| x as f64).collect::<Vec<_>>();
        let a1 = (10..26).map(|x| x as f64).collect::<Vec<_>>();

        let v0 = Mat4x4::new_r(from_slice_16(a0.as_slice()));
        let v1 = Mat4x4::new_r(from_slice_16(a1.as_slice()));

        let v2 = v0.dot(&v1);
        let v3 = Mat4x4::new_r([
            180f64, 190f64, 200f64, 210f64, 436f64, 462f64, 488f64, 514f64, 692f64, 734f64, 776f64,
            818f64, 948f64, 1006f64, 1064f64, 1122f64,
        ]);

        assert!(v2.equal(&v3));
    }
    {
        //4x4 mul 4x1
        let v0 = Mat4x1::new([1f64, 2f64, 3f64, 4f64]);
        let a0 = (0..16).map(|x| x as f64).collect::<Vec<_>>();
        let mut v1 = Mat4x4::new_r(from_slice_16(a0.as_slice()));
        let v2 = v1.dot_vec(&v0);
        assert!(v2.equal(&Mat4x1::new([20f64, 60f64, 100f64, 140f64])));
    }
    {
        //transpose
        let a0 = (0..16).map(|x| x as f64).collect::<Vec<_>>();
        let v0 = Mat4x4::new_r(from_slice_16(a0.as_slice()));
        let v1 = v0.t();
        let v2 = Mat4x4::new_r([
            0., 4., 8., 12., 1., 5., 9., 13., 2., 6., 10., 14., 3., 7., 11., 15.,
        ]);
        assert!(v1.equal(&v2));
    }
    {
        //extract upper left 3x3 matrix
        let a0 = (0..16).map(|x| x as f64).collect::<Vec<_>>();
        let v0 = Mat4x4::new_r(from_slice_16(a0.as_slice()));
        let v1 = v0.sub_rot();

        assert!(v1.equal(&Mat3x3::new_r([
            0f64, 1f64, 2f64, 4f64, 5f64, 6f64, 8f64, 9f64, 10f64,
        ])));
    }
    {
        //identity
        let v1 = Mat4x4::eye();
        assert!(v1.equal(&Mat4x4::new_r([
            1f64, 0f64, 0f64, 0f64, 0f64, 1f64, 0f64, 0f64, 0f64, 0f64, 1f64, 0f64, 0f64, 0f64,
            0f64, 1f64
        ])));
    }
}
