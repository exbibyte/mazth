use i_shape::{ ShapeType, IShape };
use i_bound::IBound;
use i_vicinity::IVicinity;

use mat::Mat3x1;
use bound::AxisAlignedBBox;

#[derive(Debug, Clone)]
pub struct RecBox {
    pub _ori: Mat3x1< f64 >,
    pub _size: f64,
    pub _bound: AxisAlignedBBox,
    pub _vicinity: f64,
}

impl RecBox {
    pub fn init( origin: &[f64], size: f64 ) -> RecBox {
        assert!( origin.len() == 3 );
        RecBox {
            _ori: Mat3x1 { _val: [ origin[0], origin[1], origin[2] ] },
            _size: size, //half of the length of box edge
            _bound: AxisAlignedBBox::init( ShapeType::BOX, &[ &origin[0..3], &[ size ] ].concat() ),
            _vicinity: 0.000001f64,
        }
    }
}

impl IShape for RecBox {
    fn get_shape_data( & self ) -> Vec< f64 > {
        vec![ self._ori[0], self._ori[1], self._ori[2], self._size ]
    }
    fn get_type( & self ) -> ShapeType {
        ShapeType::BOX
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
                ShapeType::POINT => {
                    //covered by bbox test
                    let other_shape_data = other.get_shape_data();
                    let b_off = Mat3x1 { _val: [ other_shape_data[0], other_shape_data[1], other_shape_data[2] ] };
                    return ( true, Some( b_off ) )
                },
                _ => { unimplemented!(); },
            }
        }
    }
    fn get_support( & self, v: & Mat3x1< f64 > ) -> Option< Mat3x1< f64 > > {
        if v.magnitude() != Some( 0f64 ) {
            //get a furthest point in the given direction v
            let points = [ Mat3x1 { _val: [  self._size,  self._size, self._size ] },
                           Mat3x1 { _val: [ -self._size,  self._size, self._size ] },
                           Mat3x1 { _val: [  self._size, -self._size, self._size ] },
                           Mat3x1 { _val: [ -self._size, -self._size, self._size ] },
                           Mat3x1 { _val: [  self._size,  self._size, -self._size ] },
                           Mat3x1 { _val: [ -self._size,  self._size, -self._size ] },
                           Mat3x1 { _val: [  self._size, -self._size, -self._size ] },
                           Mat3x1 { _val: [ -self._size, -self._size, -self._size ] } ];

            let furthest = points.iter()
                .map(|x| x.dot(v).unwrap() )
                .enumerate()
                .max_by(|a,b| a.1.partial_cmp(&b.1).unwrap() ).unwrap();

            let o = self._ori.plus( &points[furthest.0] ).expect( "support operation unsuccessful.");
            Some( o )
        } else {
            None
        }
    }
}

impl IVicinity< f64 > for RecBox {
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
