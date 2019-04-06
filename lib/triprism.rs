///triangular prism (5 faces: 2 triangles, 3 squares)

use i_shape::{ ShapeType, IShape };
use i_bound::IBound;
use i_vicinity::IVicinity;

use mat::Mat3x1;
use bound::AxisAlignedBBox;

#[derive(Debug, Clone, Default)]
pub struct TriPrism {
    
    ///base
    pub _tri_base: [ Mat3x1< f64 >; 3 ], 

    ///base + height offset in normal direction of base
    pub _tri_base2: [ Mat3x1< f64 >; 3 ],

    ///normal of the triangle base, scaled with height
    pub _normal_height: Mat3x1< f64 >,
    
    pub _bound: AxisAlignedBBox,
    
    pub _vicinity: f64,
}

impl TriPrism {
    /// initialize with tribase: base vertices in ccw order
    pub fn init( tri_base: &[f64], height: f64 ) -> TriPrism {
        
        assert!( tri_base.len() == 9 );

        let v0 = Mat3x1 { _val: [ tri_base[0], tri_base[1], tri_base[2] ] };
        let v1 = Mat3x1 { _val: [ tri_base[3], tri_base[4], tri_base[5] ] };
        let v2 = Mat3x1 { _val: [ tri_base[6], tri_base[7], tri_base[8] ] };

        let d1 = v1.minus( &v0 ).unwrap();
        let d2 = v2.minus( &v0 ).unwrap();
        let normal = d1.cross( &d2 ).unwrap().normalize().unwrap();
        let h_offset = normal.scale( height ).unwrap();

        let v00 = v0.plus( &h_offset ).unwrap();
        let v11 = v1.plus( &h_offset ).unwrap();
        let v22 = v2.plus( &h_offset ).unwrap();

        let base = [ v0, v1, v2 ];
        let base2 = [ v00, v11, v22 ];
        
        use std::cmp::Ordering::*;

        let xs = [ base[0][0], base[1][0], base[2][0],
                   base2[0][0], base2[1][0], base2[2][0] ];

        let ys = [ base[0][1], base[1][1], base[2][1],
                   base2[0][1], base2[1][1], base2[2][1] ];

        let zs = [ base[0][2], base[1][2], base[2][2],
                   base2[0][2], base2[1][2], base2[2][2] ];
        
        let x_min = *xs.iter().min_by( |a,b| a.partial_cmp(b).unwrap_or(Equal) ).unwrap();
        let x_max = *xs.iter().max_by( |a,b| a.partial_cmp(b).unwrap_or(Equal) ).unwrap();

        let y_min = *ys.iter().min_by( |a,b| a.partial_cmp(b).unwrap_or(Equal) ).unwrap();
        let y_max = *ys.iter().max_by( |a,b| a.partial_cmp(b).unwrap_or(Equal) ).unwrap();

        let z_min = *zs.iter().min_by( |a,b| a.partial_cmp(b).unwrap_or(Equal) ).unwrap();
        let z_max = *zs.iter().max_by( |a,b| a.partial_cmp(b).unwrap_or(Equal) ).unwrap();
       
        TriPrism {
            _tri_base: base,
            _tri_base2: base2,
            _normal_height: h_offset,
            _bound: AxisAlignedBBox::init( ShapeType::RECT, &[ x_min, y_min, z_min,
                                                               x_max, y_max, z_max ] ),
            _vicinity: 0.000001f64,
        }
    }
}

impl IShape for TriPrism {
    fn get_shape_data( & self ) -> Vec< f64 > {
        vec![ self._tri_base[0][0], self._tri_base[0][1], self._tri_base[0][2],
              self._tri_base[1][0], self._tri_base[1][1], self._tri_base[1][2],
              self._tri_base[2][0], self._tri_base[2][1], self._tri_base[2][2],
              self._normal_height[0], self._normal_height[1], self._normal_height[2] ]
    }
    fn get_type( & self ) -> ShapeType {
        ShapeType::TRI_PRISM
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

                    let other_shape_data = other.get_shape_data();
                    let other_point = Mat3x1 { _val: [ other_shape_data[0], other_shape_data[1], other_shape_data[2] ] };

                    //test point aginst 5 half spaces from facets of the tri_prism to determine if point is inside the tri_prism

                    let n = self._normal_height;
                    
                    let tests = vec![ ( self._tri_base[0], n.scale(-1.).unwrap() ),
                                      ( self._tri_base2[0], n ),
                                      ( self._tri_base[0], self._tri_base[1].minus(&self._tri_base[0]).unwrap().cross( &n ).unwrap() ),
                                      ( self._tri_base[1], self._tri_base[2].minus(&self._tri_base[1]).unwrap().cross( &n ).unwrap() ),
                                      ( self._tri_base[2], self._tri_base[0].minus(&self._tri_base[2]).unwrap().cross( &n ).unwrap() ) ];

                    let is_inside = tests.iter()
                        .all(|(vert,normal)| !(other_point.minus(vert).unwrap().dot(normal).unwrap() > 0.) );

                    if is_inside {
                        ( true, Some( other_point ) )
                    } else {
                        ( false, None )
                    }
                },
                _ => { unimplemented!(); },
            }
        }
    }
    fn get_support( & self, v: & Mat3x1< f64 > ) -> Option< Mat3x1< f64 > > {
        if v.magnitude() != Some( 0f64 ) {
            //get a furthest point in the given direction v
            let points = [ self._tri_base[0], self._tri_base[1], self._tri_base[2],
                           self._tri_base2[0], self._tri_base2[1], self._tri_base2[2] ];

            let furthest = points.iter()
                .map(|x| x.dot(v).unwrap() )
                .enumerate()
                .max_by(|a,b| a.1.partial_cmp(&b.1).unwrap() ).unwrap();

            let o = points[furthest.0].clone();

            Some( o )
        } else {
            None
        }
    }
}

impl IVicinity< f64 > for TriPrism {
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
