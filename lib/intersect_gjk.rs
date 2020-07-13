use ndarray::prelude::*;

//based on reference tutorial from http://www.dyn4j.org/2010/04/gjk-gilbert-johnson-keerthi/

use mat::*;
use shape::{IShape, ShapeType};

fn support(a: &dyn IShape, b: &dyn IShape, v: &Matrix1D) -> Option<Matrix1D> {
    let p0 = match a.get_support(&v) {
        Some(o) => o,
        _ => return None,
    };
    let v_oppose = v * -1f64;
    let p1 = match b.get_support(&v_oppose) {
        Some(o) => o,
        _ => return None,
    };
    let p10 = &p0 - &p1;
    Some(p10)
}

fn pass_minkowski_origin(last_vert: &Matrix1D, support: &Matrix1D) -> bool {
    // println!( "last vert dot product: {}", last_vert.dot( &support ).unwrap() );
    last_vert.dot(support) > 0f64
}

fn contains_minkowski_origin(simplex: &mut Vec<Matrix1D>, support: &mut Matrix1D) -> bool {
    let a = simplex.last().unwrap().clone();
    let ao = &a * -1f64;
    if simplex.len() == 3 {
        //triangle case
        let ref b = simplex[1];
        let ref c = simplex[0];
        let ab = b - &a;
        let ac = c - &a;
        let ab_normal = cross_vec_1d(&(cross_vec_1d(&ac.view(), &ab.view())).view(), &ab.view());
        let ac_normal = cross_vec_1d(&(cross_vec_1d(&ab.view(), &ac.view())).view(), &ac.view());
        if ab_normal.dot(&ao) > 0f64 {
            //remove c and set new direction to ab_normal
            let simplex_new = vec![simplex[1].clone(), simplex[2].clone()];
            *simplex = simplex_new;
            *support = ab_normal.clone();
        } else if ac_normal.dot(&ao) > 0f64 {
            //remove b and set new direction to ac_normal
            let simplex_new = vec![simplex[0].clone(), simplex[2].clone()];
            *simplex = simplex_new.clone();
            *support = ac_normal.clone();
        } else {
            //minkowski origin is enclosed by the triangle
            return true;
        }
    } else {
        //line segment case
        //set direction towards minkowski origin
        let ref b = simplex[0];
        let ab = b - &a;
        let ab_normal = cross_vec_1d(&cross_vec_1d(&ab.view(), &ao.view()).view(), &ao.view());
        if mag_vec_l2_1d(&ab_normal.view()) == 0f64 {
            return true;
        } else {
            *support = ab_normal.clone();
        }
    }
    false
}

pub fn query_intersect(a: &dyn IShape, b: &dyn IShape) -> Option<bool> {
    match (a.get_type(), b.get_type()) {
        (ShapeType::Sphere, ShapeType::Sphere) => {}
        //todo
        // (ShapeType::POINT,ShapeType::BOX) => {},
        // (ShapeType::BOX,ShapeType::POINT) => {},
        _ => {
            panic!("unsupported shape type");
        }
    }
    //set initial minkowski vertex from an arbitrary support vector
    let mut d = arr1(&[-1f64, 0f64, 0f64]);
    let mut simplex = vec![];
    {
        let sup = support(a, b, &d).unwrap();
        simplex.push(sup);
    }

    d = d * -1f64;
    loop {
        // println!( "support vector: {:?}", d );
        {
            let sup = support(a, b, &d).unwrap();
            simplex.push(sup);
        }
        assert!(simplex.len() <= 3, "simplex vertices count unexpected");
        // println!( "simplex len: {}", simplex.len() );
        if !pass_minkowski_origin(simplex.last().unwrap(), &d) {
            // println!( "new vert not pass origin" );
            return Some(false);
        } else {
            if contains_minkowski_origin(&mut simplex, &mut d) {
                return Some(true);
            }
        }
    }
}
