// extern crate ndarray;
// extern crate ndarray_rand;
// extern crate approx;

use ndarray::prelude::*;
pub use ndarray::{Array, Ix3};
use ndarray::{aview0, aview1, arr2, Axis};
    
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
    
#[allow(unused_imports)]
use std::f32;
use std::f64;

#[allow(unused_imports)]
use std::ops::Div;
#[allow(unused_imports)]
use std::ops::Index;
#[allow(unused_imports)]
use std::ops::IndexMut;
#[allow(unused_imports)]
use std::ops::Mul;

pub type Matrix= Array::<f64, Ix2>;
pub type MatrixView<'a> = ArrayView::<'a, f64, Ix2>;

pub const eps: f64 = 1e15;

pub fn cross_vec(a: & MatrixView, b: & MatrixView ) -> Matrix {
    assert!(a.shape().len()==2);
    assert!(b.shape().len()==2);
    assert!(a.shape()[0]>=3);
    assert!(b.shape()[0]>=3);
    assert!(a.shape()[1]==1);
    assert!(b.shape()[1]==1);
    let mut ret = Matrix::zeros((4,1));
    array![ [ a[[1,0]] * b[[2,0]] - b[[1,0]] * a[[2,0]] ],
            [ - a[[0,0]] * b[[2,0]] + b[[0,0]] * a[[2,0]] ],
            [ a[[0,0]] * b[[1,0]] - b[[0,0]] * a[[1,0]] ],
            [ 0.] ]
}

pub fn mag_vec_l2(a: & MatrixView) -> f64 {
    (a*a).sum().sqrt()
}

pub fn mag_vec3_l2(a: & MatrixView) -> f64 {
    assert!(a.shape()[0] >= 3);
    let s = a.slice(s![0..3,..]);
    s.t().dot(&s).sum().sqrt()
}

pub fn normalize_vec_l2(a: & MatrixView) -> Matrix {
    let m = mag_vec_l2(a);
    let factor = 1.0/(m + eps);
    let b = a.to_owned();
    b * factor
}


#[test]
fn test(){
    {
        let a = Array::random((4, 1), Uniform::new(0., 10.));
        let b = Array::random((4, 1), Uniform::new(0., 10.));
        let c = cross_vec(&a.view(), &b.view());
        assert_eq!( c.shape(), &[4,1] );
    }
    {
        let a = Array::random((4, 1), Uniform::new(0., 10.));
        let b = Array::random((4, 1), Uniform::new(0., 10.));
        let c = &a+&b;
        for i in 0..4 {
            relative_eq!(a[[i,0]] + b[[i,0]], c[[i,0]], epsilon = f64::EPSILON);
        }
    }
    {
        let a = Array::random((4, 1), Uniform::new(0., 10.));
        let b = Array::random((4, 1), Uniform::new(0., 10.));
        let c = &a*&b;
        
        assert_eq!( c.shape(), &[4,1] );
        for i in 0..4 {
            relative_eq!(a[[i,0]] * b[[i,0]], c[[i,0]], epsilon = f64::EPSILON);
        }
    }
    {
        let a = Array::random((4, 1), Uniform::new(0., 10.));
        let b = Array::random((4, 1), Uniform::new(0., 10.));
        let c = a.dot(&b.t());
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(c[[i,j]], a[[i,0]] * b[[j,0]]);
            }
        }
    }
    {
        let a = Array::random((3, 1), Uniform::new(0., 10.));
        let m = mag_vec3_l2(&a.view());
        relative_eq!(m, (0..3).map(|i| a[[i,0]]*a[[i,0]]).sum::<f64>().sqrt(), epsilon = f64::EPSILON);
    }
    {
        let aa = array![[4.0], [5.0], [2.0], [3.0]];
        let m = normalize_vec_l2(&aa.view());
        assert!( m.shape() == &[4,1] );
        let factor = 1.0f64/(4.0f64 * 4.0 + 5.0 * 5.0 + 2.0 * 2.0 + 3.0 * 3.0).sqrt();
        let expect = array![[ 4.0/factor, 5.0/factor, 2.0/factor, 3.0/factor ]];
        for i in 0..4 {
            relative_eq!(m[[i,0]], expect.t()[[i,0]], epsilon = f64::EPSILON);
        }
    }    
}
