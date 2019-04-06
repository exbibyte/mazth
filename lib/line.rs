use i_shape::{ ShapeType, IShape };
use i_bound::IBound;
use i_vicinity::IVicinity;

use mat::Mat3x1;
use i_comparable::IComparableError;
use bound::AxisAlignedBBox;

#[derive(Debug, Clone)]
pub struct Line3 {
    pub _a: Mat3x1< f64 >,
    pub _b: Mat3x1< f64 >,
    pub _bound: AxisAlignedBBox,
    pub _vicinity: f64,
}

impl Line3 {
    pub fn init( a: &[f64], b: &[f64] ) -> Line3 {
        assert!( a.len() == 3 );
        assert!( b.len() == 3 );

        let xs = vec![a[0], b[0]];
        let ys = vec![a[1], b[1]];
        let zs = vec![a[2], b[2]];

        use std::cmp::Ordering::*;
                
        let x_min = *xs.iter().min_by( |a,b| a.partial_cmp(b).unwrap_or(Equal) ).unwrap();
        let x_max = *xs.iter().max_by( |a,b| a.partial_cmp(b).unwrap_or(Equal) ).unwrap();

        let y_min = *ys.iter().min_by( |a,b| a.partial_cmp(b).unwrap_or(Equal) ).unwrap();
        let y_max = *ys.iter().max_by( |a,b| a.partial_cmp(b).unwrap_or(Equal) ).unwrap();

        let z_min = *zs.iter().min_by( |a,b| a.partial_cmp(b).unwrap_or(Equal) ).unwrap();
        let z_max = *zs.iter().max_by( |a,b| a.partial_cmp(b).unwrap_or(Equal) ).unwrap();
        
        Line3 {
            _a: Mat3x1 { _val: [ a[0], a[1], a[2] ] },
            _b: Mat3x1 { _val: [ b[0], b[1], b[2] ] },
            _bound: AxisAlignedBBox::init( ShapeType::RECT, &[ x_min, y_min, z_min,
                                                               x_max, y_max, z_max ] ),
            _vicinity: 0.000001f64,
        }
    }
}

impl IShape for Line3 {
    fn get_shape_data( & self ) -> Vec< f64 > {
        vec![ self._a[0], self._a[1], self._a[2],
              self._b[0], self._b[1], self._b[2] ]
    }
    fn get_type( & self ) -> ShapeType {
        ShapeType::LINE
    }
    fn get_bound( & self ) -> &IBound {
        &self._bound
    }
    // this shall test for intersection of bounding shapes first before procedding to test intersection using algorithms of higher complexity
    fn get_intersect( & self, other: & IShape ) -> ( bool, Option< Mat3x1< f64 > > ){
        if !self.get_bound().intersect( other.get_bound() ){
            return ( false, None )
        }else{
            match other.get_type() {
                ShapeType::TRI_PRISM => {
                    other.get_intersect( self )
                },
                _ => { unimplemented!(); },
            }
        }
    }
    fn get_support( & self, _v: & Mat3x1< f64 > ) -> Option< Mat3x1< f64 > > {
        unimplemented!();
    }
}

impl IVicinity< f64 > for Line3 {
    fn set_vicinity( & mut self, epsilon: f64 ) {
        self._vicinity = epsilon.abs();
    }
    fn within_vicinity( & self, a: f64, b: f64 ) -> bool {
        if a + self._vicinity >= b &&
           a - self._vicinity <= b {
            true
        } else {
            false
        }
    }
}
