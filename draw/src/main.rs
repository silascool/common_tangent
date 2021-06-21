mod draw;
use libary::polygon_generator;
use libary::ghs;
use libary::melkmanns;
use libary::util::{Point};
fn main() {
    
    let polys1 = polygon_generator::gen_polygon(8);
    draw::draw_aw_stepbystep( &polys1, &[-1, -1]);
}



























fn _draw_ghs_test (){
    let a = Point{x:-1.0 ,y: 0.0};
    let b = Point{x:-1.0 ,y:-1.0};
    let c = Point{x:0.0 ,y:-2.0};
    let d = Point{x:2.0 ,y: 0.0};
    let e = Point{x:0.0 ,y: 0.0};
    let f = Point{x:2.0 ,y: 0.0};
    let g = Point{x:2.0 ,y: 1.0};
    let _poly = vec!(a,b,c,d,e,f,g,a);
    
    let p = Point{x:0.0, y: 0.0};
    let o = Point{x:3.0, y: 0.0};
    let l = Point{x:-4.0, y: 1.0};
    let y = Point{x:6.0, y: 2.0};
    let s = Point{x:-8.0, y: 3.0};
    let f = Point{x:-8.0, y: 0.0};
    let poly2 = vec!(p,o,l,y,s, f, p);

    let a = Point{x:-1.0, y: 20.0};
    let b = Point{x:0.0, y:20.0};
    let c = Point{x:0.0, y: 21.0};
    let d = Point{x:-1.0, y: 21.0};

    let poly1 = vec!(a,b,c,d,a);
    let deque = melkmanns::algorithm_hull(&poly2);
    let deque1 = melkmanns::algorithm_hull1(&poly2);
    let polys = vec!(poly1, poly2);
   
    for p in deque {
        println! ("cc {}, {}", p.x, p.y);
    }
    for p in deque1 {
        println! ("c {}, {}", p.x, p.y);
    }
    //  draw::draw_deque_static(&polys, true);
    }
#[test]
fn name() {
    let polys = polygon_generator::gen_polygon(10);
    let hull = melkmanns::algorithm_hull(&polys[0]);
    let s0 = hull.as_slices();
    let mut p0 = s0.0.to_vec();
    p0.extend_from_slice(s0.1);
    //p0.reverse();   

    let hull = melkmanns::algorithm_hull(&polys[1]);
    let s1 = hull.as_slices();
    let mut p1 = s1.0.to_vec();
    p1.extend_from_slice(s1.1);
 
    let (l_arr, u_arr) = ghs::generate_u_l (&p1);
    
    p1.reverse(); 
    
    let (l_arr_rev, u_arr_rev) = ghs::generate_u_l (&p1);
    for i in 0..l_arr.len() {
        assert_eq! (l_arr[i].x, u_arr_rev[ u_arr_rev.len()-1-i ].x );
        assert_eq! (l_arr[i].y, u_arr_rev[ u_arr_rev.len()-1-i ].y );
    }
}
