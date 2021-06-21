use std::collections::VecDeque;
use super::util::{Point, plane, is_left_of, is_right_of, right_colinear_left};

///implemenation of  https://www.ime.usp.br/~walterfm/cursos/mac0331/2006/melkman.pdf
///Constuct the convex hull of a simple polygon
/// using VecDeque, should maybe make a variant where i don use any preconstructed types

pub fn algorithm_hull (polygon: &Vec<Point>) -> VecDeque<Point> {
    let mut d: VecDeque<Point> = VecDeque::new();
    let v1 = polygon[0]; let v2 = polygon[1]; let v3 = polygon[2];
    if plane (v1, v2, v3) {
        d.push_back(v2); 
        d.push_back(v1);
    } else {
        d.push_back(v1);
        d.push_back(v2); 
    }
    d.push_back(v3);
    d.push_front(v3);
    //println! ("hej");
    for i in 3..(polygon.len()-1) {
    // until (v, d_b, d_{b+1}) < 0 or (d_{t-1}, d_t, v) < 0. is tranlated to this
        let v = polygon[i];
        if !is_left_of(d[d.len()-2], d[d.len()-1], v)  || !is_right_of(d[1], d[0], v){
            while !is_right_of(d[1], d[0], v) {
                //println! ("pop front {}, {}", v.x, v.y);
                d.pop_front();
            }
            d.push_front(v);
            while !is_left_of(d[d.len()-2], d[d.len()-1], v)  {
                //println! ("pop back {}, {}", v.x, v.y);
                d.pop_back();
            }
            d.push_back(v);
            //println! ("counterclockwise  {}, {}",d[0].x, d[0].y);
        }
    }
    return d
}


pub fn multi_melk( polys: &Vec<Vec<Point>>) -> [VecDeque<Point>; 2] {
    [algorithm_hull(&polys[0]), algorithm_hull(&polys[1])]
}
pub fn algorithm_hull1 (polygon: &Vec<Point>) -> VecDeque<Point> {
    let mut d: VecDeque<Point> = VecDeque::new();
    let v1 = polygon[0]; let v2 = polygon[1]; let v3 = polygon[2];
    if plane (v1, v2, v3) {
        d.push_back(v1);
        d.push_back(v2); 
    } else {
        d.push_back(v2);
        d.push_back(v1);
    }
    d.push_back(v3);
    d.push_front(v3);
    
    for i in 3..(polygon.len()-1) {
    // until (v, d_b, d_{b+1}) < 0 or (d_{t-1}, d_t, v) < 0. is tranlated to this
        let v = polygon[i];
        if !plane(v, d[0], d[1])  || !plane(d[d.len()-2], d[d.len()-1], v) {    
            while !plane(d[d.len()-2], d[d.len()-1], v) {
                d.pop_back();
            }
            d.push_back(v);
            while right_colinear_left(v, d[0], d[1]) != 1  {
                d.pop_front();
            }
            d.push_front(v);
            println! ("clockwise  {}, {}",d[0].x, d[0].y);
        }
    }
    return d
}


#[test]
pub fn melk_test () {

    ///
    ///             g
    ///             |
    ///      a  e---d
    ///      |     /
    ///      b    /
    ///        \ /
    ///         c

    let a = Point{x:-1.0 ,y: 0.0};
    let b = Point{x:-1.0 ,y:-1.0};
    let c = Point{x:0.0 ,y:-2.0};
    let d = Point{x:2.0 ,y: 0.0};
    let e = Point{x:0.0 ,y: 0.0};
    let f = Point{x:2.0 ,y: 0.0};
    let g = Point{x:2.0 ,y: 1.0};
    let poly = vec!(a,b,c,d,e,f,g,a);
    
    let deque = algorithm_hull(&poly);
    println! ("{}", deque.len());
    for p in &deque {
        println! ("{}, {}", p.x, p.y);
       }
    
    assert_eq! (deque[0].x, g.x);
    assert_eq! (deque[0].y, g.y);
    assert_eq! (deque[1].x, a.x);
    assert_eq! (deque[1].y, a.y);
    assert_eq! (deque[2].x, b.x);
    assert_eq! (deque[2].y, b.y);
    assert_eq! (deque[3].x, c.x);
    assert_eq! (deque[3].y, c.y);
    assert_eq! (deque[4].x, d.x);
    assert_eq! (deque[4].y, d.y);
    assert_eq! (deque[5].x, g.x);
    assert_eq! (deque[5].y, g.y);
    
}
