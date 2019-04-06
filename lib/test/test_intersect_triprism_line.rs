use i_shape::IShape;
use i_comparable::IComparableError;

use triprism::TriPrism;
use line::Line3;

#[test]
fn test_intersect_line() {

    //intersection
    {
        let a = TriPrism::init( &[0., 0., 0.,
                                  1., 0., 0.,
                                  1., 1., 0. ],
                                  1. );

        let b = Line3::init( &[ 0.25, 0.25, 0. ], &[ 1., 1., 0. ], );
        
        match a.get_intersect( &b ) {
            ( true, Some(loc) ) => {},
            _ => panic!("unexpected result for triprism line intersection" ),
        }
    }
    //intersection
    {
        let a = TriPrism::init( &[0., 0., 0.,
                                  1., 0., 0.,
                                  1., 1., 0. ],
                                  1. );

        let b = Line3::init( &[ 0.5, -50., 0. ], &[ 0.5, 50., 0. ], );
        
        match a.get_intersect( &b ) {
            ( true, Some(loc) ) => {},
            _ => panic!("unexpected result for triprism line intersection" ),
        }
    }
    //intersection
    {
        let a = TriPrism::init( &[0., 0., 0.,
                                  1., 0., 0.,
                                  1., 1., 0. ],
                                  1. );

        let b = Line3::init( &[ 0.25, 0.25, 0.5 ], &[ 0.26, 0.26, 0.5 ], );
        
        match a.get_intersect( &b ) {
            ( true, Some(loc) ) => {},
            _ => panic!("unexpected result for triprism line intersection" ),
        }
    }
    
    //intersection, flipped
    {
        let a = TriPrism::init( &[0., 0., 0.,
                                  1., 0., 0.,
                                  1., 1., 0. ],
                                  1. );

        let b = Line3::init( &[ 0.25, 0.25, 0. ], &[ 1., 1., 0. ], );
        
        match b.get_intersect( &a ) {
            ( true, Some(loc) ) => {},
            _ => panic!("unexpected result for triprism line intersection" ),
        }
    }
    //intersection, flipped
    {
        let a = TriPrism::init( &[0., 0., 0.,
                                  1., 0., 0.,
                                  1., 1., 0. ],
                                  1. );

        let b = Line3::init( &[ 0.5, -50., 0. ], &[ 0.5, 50., 0. ], );
        
        match b.get_intersect( &a ) {
            ( true, Some(loc) ) => {},
            _ => panic!("unexpected result for triprism line intersection" ),
        }
    }
    //intersection, flipped
    {
        let a = TriPrism::init( &[0., 0., 0.,
                                  1., 0., 0.,
                                  1., 1., 0. ],
                                  1. );

        let b = Line3::init( &[ 0.25, 0.25, 0.5 ], &[ 0.26, 0.26, 0.5 ], );
        
        match b.get_intersect( &a ) {
            ( true, Some(loc) ) => {},
            _ => panic!("unexpected result for triprism line intersection" ),
        }
    }
    
    //no intersection
    {
        let a = TriPrism::init( &[0., 0., 0.,
                                  1., 0., 0.,
                                  1., 1., 0. ],
                                  1. );

        let b = Line3::init( &[ 0.25, 0.25, 1.5 ], &[ 1., 1., 1.5 ], );
        
        match a.get_intersect( &b ) {
            ( true, Some(loc) ) => { panic!("unexpected result for triprism line intersection" ); },
            _ => {},
        }
    }

    //no intersection
    {
        let a = TriPrism::init( &[0., 0., 0.,
                                  1., 0., 0.,
                                  1., 1., 0. ],
                                  1. );

        let b = Line3::init( &[ 0., -5., 0.5 ], &[ 50., 45., 0.5 ], );
        
        match a.get_intersect( &b ) {
            ( true, Some(loc) ) => { panic!("unexpected result for triprism line intersection" ); },
            _ => {},
        }
    }

}
