use mat::*;
use quat::Quat;
use std::f64::consts::PI;

#[test]
fn test_quat() {
    {
        //convert axis angle to quaternion representation and back
        let axis = Mat3x1::new([1., 2., 3.]);

        let axis_normalize = axis.normalize_l2();
        let q = Quat::init_from_axis_angle_degree(axis, 90.);
        let (a, angle) = q.to_axis_angle();
        assert!(a.equal(&axis_normalize));
        assert!((angle / PI * 180. - 90.).abs() < 1e-9);
    }
    {
        //convert axis angle to quaternion representation and back
        let axis = Mat3x1::new([1., 2., 3.]);

        let axis_normalize = axis.normalize_l2();
        let q = Quat::init_from_axis_angle_degree(axis, 370.);
        let (a, angle) = q.to_axis_angle();
        assert!(a.equal(&axis_normalize));
        assert!((angle / PI * 180. - 10.).abs() < 1e-9);
    }
    {
        //convert axis angle to quaternion representation and back
        let axis = Mat3x1::new([1., 2., 3.]);

        let axis_normalize = axis.normalize_l2();
        let q = Quat::init_from_axis_angle_degree(axis, -33.);
        let (a, angle) = q.to_axis_angle();
        assert!(
            ((angle / PI * 180. - (360. - 33.)).abs() < 1e-9 && a.equal(&axis_normalize))
                || ((angle / PI * 180. + (360. - 33.)).abs() < 1e-9
                    && (&a * -1.).equal(&axis_normalize))
        );
    }
    {
        //compute rotation using quaternion
        //rotate a vector using the rotation matrix and compare to rotation using quaternions
        let p = Mat3x1::new([1., 5., -3.]);
        let axis = Mat3x1::new([1., 0., 0.]);

        let axis_normalize = axis.normalize_l2();
        let q = Quat::init_from_axis_angle_degree(axis, 90.);
        let (a, angle) = q.to_axis_angle();
        assert!(a.equal(&axis_normalize));
        assert!(angle / PI * 180. == 90.);

        let row_major = true;
        let rot = q.to_rotation_matrix();

        assert!(rot.equal(&Mat4x4::new_r([
            1., 0., 0., 0., 0., 0., -1., 0., 0., 1., 0., 0., 0., 0., 0., 1.
        ])));

        let ans_1 = rot.dot_vec(&Mat4x1::new([p[0], p[1], p[2], 1.]));
        let ans_2 = q.rotate_vector(p);
        assert!(ans_2.equal(&Mat3x1::new([ans_1[0], ans_1[1], ans_1[2]])));
    }
}
