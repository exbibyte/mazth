#[test]
fn test_mat4x1() {
    use mat::*;

    let v0 = Mat4x1::new([10f64; 4]);
    let v1 = Mat4x1::new([2f64; 4]);
    {
        let v2 = &v0 * &v1;
        assert!(v2.equal(&Mat4x1::new([20f64; 4])));
    }
    {
        let v2 = &v0 / &v1;
        assert!(v2.equal(&Mat4x1::new([5f64; 4])));
    }
    {
        let v2 = &v0 + &v1;
        assert!(v2.equal(&Mat4x1::new([12f64; 4])));
    }
    {
        let v2 = &v0 - &v1;
        assert!(v2.equal(&Mat4x1::new([8f64; 4])));
    }

    assert!(v1.norm_l2() == ((2f64 * 2f64) * 4f64).sqrt());
    {
        let v2 = v1.normalize();
        assert!(v2.equal(&Mat4x1::new([2f64 / (2f64 * 2f64 * 4f64).sqrt(); 4])));
    }
    {
        let v2 = &v1 * 4.;
        assert!(v2.equal(&Mat4x1::new([8f64; 4])));
    }
}
