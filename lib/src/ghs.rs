use super::util::{Point, plane, find_left_n_right_xtreme};
use std::cmp::Ordering;
use std::collections::VecDeque;


/// Creates U and L from Polygon P, U the upper part of P and L is the lower part.
/// This function assumes P has "positi v omløbsretnig"
pub fn generate_u_l (p: &Vec<Point>) -> (Vec<Point>, Vec<Point>) {
    
    let (l, h) = find_left_n_right_xtreme( &p );
    // never read
    let mut u_arr: Vec<Point> = Vec::new();
    let mut l_arr: Vec<Point> = Vec::new();
    if l < h {
        u_arr = p[h..].to_vec();
        if l != 0 { 
            u_arr.extend_from_slice( &p[1..l+1] );
        }// OBS if l == 0 then h != p.len()-1, since l<h h cant be at the end
        l_arr = p[l..h+1].to_vec();
    } else {
        l_arr = p[l..].to_vec();
        if h != 0 {
            l_arr.extend_from_slice( &p[1..h+1] )
        }
        u_arr = p[h..l+1].to_vec();
        
    }
/*
    for point in &l_arr {
        print! ("l {} {}", point.x, point.y);
    }
    println! ("");
    for point in &u_arr {
        print! ("u {} {}", point.x, point.y);
    }
    
    println! ("");
    */
    return (l_arr, u_arr)
    
}


fn binary_search(x: Point, chain: &Vec<Point>, is_upper: bool) -> Option<usize> {
    let mut size = chain.len()-1;
    if size == 0 {
        return None
    }
    let mut base = 0usize; 
    loop {
        
        let half = size / 2;
        let mid = base + half;
        match (is_upper, chain[mid].x >= x.x, chain[mid+1].x >= x.x) {
            (true, _, true) => { base = mid; },
            (false, _, false) => { base = mid; },
            (true, true, false) => { return Some(mid) },
            (false, false, true) => { return Some(mid) },
            _ => (),
        }
        
        if size == 1 {
            return None
        }
        size -= half;
    };
}


fn is_tangent (index: usize, x: Point, chain : &Vec<Point>, is_right: bool) -> Ordering {
    let lastelem = chain.len()-1;
    match index {
        0 => {
            Ordering::Equal},
        index if index < lastelem => {
            let a = plane(x, chain[index], chain[index+1]);
            let b = plane(x, chain[index], chain[index-1]);
            match (a,b) {
                    (true, false) => if is_right {Ordering::Greater} else {Ordering::Less}
                    (false, true) => if is_right {Ordering::Less} else {Ordering::Greater}
                    (true, true) => if is_right {Ordering::Greater} else {Ordering::Equal}
                    (false, false) => if is_right {Ordering::Equal} else {Ordering::Less},
            }
        }
        _ => {
            Ordering::Equal
        }
    }
}

fn binary_tangent_search (x: Point, chain: &Vec<Point>, is_right: bool) -> Option<usize> {
        let mut size = chain.len();
        if size == 0 {
            return None
        }
        let mut base = 0usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            // mid is always in [0, size), that means mid is >= 0 and < size.
            // mid >= 0: by definition
            // mid < size: mid = size / 2 + size / 4 + size / 8 ...
            let cmp = is_tangent(mid, x, chain, is_right);
            //match cmp {
            //    Ordering println! ("{}", )
            //}
            match cmp {
                Ordering::Equal => { base = mid; break; }
                Ordering::Greater => { base = mid }
                Ordering::Less => {}
            }
            size -= half;
        }
        // base is always in [0, size) because base <= mid.
        let cmp = is_tangent(base, x, chain, is_right);
        if cmp == Ordering::Equal { Some(base) } else { None }
    }

pub fn tangent_from_point(x: Point, u_chain: &Vec<Point>, l_chain: &Vec<Point>, is_right_tangent: bool) -> Option<(bool,usize)> {
    let mut is_upper: Option<bool> = None;
    match x {
        x if x.x < l_chain[0].x => {//left
            is_upper =  Some(!is_right_tangent);
            }
        x if x.x > u_chain[0].x => {//right
            is_upper = Some(is_right_tangent);
        }
        x if x.x > l_chain[0].x && x.x < u_chain[0].x => {// middel
                let xu = binary_search(x, u_chain, true)?;
                //skal være plane istedet
                if plane(u_chain[xu], u_chain[xu+1], x) {
                    is_upper = Some(true);
                }
                let xl = binary_search(x, l_chain, false)?;
                if plane(l_chain[xl], l_chain[xl+1], x) {
                    is_upper = Some(false);
                }
            }
        _ => {}
    };
    match is_upper {
        Some(true) => binary_tangent_search(x, u_chain, is_right_tangent).map(|s| (true, s)),
        Some(false) =>  binary_tangent_search(x, l_chain, is_right_tangent).map(|s| (false, s)),
        None => None,
    }
}

pub fn naive_tangent_search(polys: &Vec<VecDeque<Point>>) -> Vec<Option<(usize,usize)>> {
    let s0 = polys[0].as_slices();
    let mut p0 = s0.0.to_vec();
    p0.extend_from_slice(s0.1);

    let s1 = polys[1].as_slices();
    let mut p1 = s1.0.to_vec();
    p1.extend_from_slice(s1.1);
    let (u0, l0) = generate_u_l(&p0);
    let (u1, l1) = generate_u_l(&p1);
    println! ("point upper ({}, {})  lower ({}, {})", u0[0].x, u0[0].y, l0[0].x, l0[0].y);
    println! ("u0.len() {}, l0.len() {}", u0.len(), l0.len());
    println! ("u1.len() {}, l1.len() {}", u1.len(), l1.len());
    //this is respekt to an element in po 
    /*let pindex = match (u0[0].x < u1[0].x, l0[0].x < l1[0].x) {
        //p0 is left of p1, therefor u0 to u1 is an left tangent
        (true, true) => 
        //p0 is right of p1,  therefor u0 to u1 is an right tangent
        (false, false) => Some(true),
        //p0 is incased in p1 or p1 is incased in p0
        _ => None,
    };*/
    let mut pindex = 0;
    let mut point = p0[pindex];
    println! ("point: {}, {}", point.x, point.y);
    let mut right = true;
    let mut tangents: Vec<Option<(usize,usize)>> = Vec::new();
    println! ("foer loop");
    for j in 0..1 {
        tangents[j] = 'inner: loop {
            let (tangent, index1) = match tangent_from_point(point, &u1, &l1, right) {
                Some((true, i)) => (Some(u1[i]), Some(i)),
                Some((false, i)) => (Some(l1[i]), Some(i)),
                None => (None, None)
            };
            match tangent {
                None => {
                    pindex = u0.len()-1;
                    point = u0[pindex];
                    },
                Some(v) => {
                    println! ("potentiel tangent: {}, {}", v.x, v.y);
                    let (tangents_tangent, index2) = match tangent_from_point(v, &u0, &l0, !right) {
                        Some((true, i)) => (Some(u0[i]), Some(i)),
                        Some((false, i)) => (Some(l0[i]), Some(i)),
                        None => (None, None),
                        };
                    match tangents_tangent {
                        None => {println! ("hej"); 
                                break 'inner None;},
                        Some(p) => {
                            if point.x == p.x && point.y == p.y {
                                break 'inner Some((pindex, index1.unwrap()));
                            } else {
                                point = p;
                                println! ("ptangents_tangent: {}, {}", p.x, p.y);
                                pindex = index2.unwrap();
                            }
                        }
                    }
                }
            }
        };
    
        right = !right;
    }

    return tangents
}

pub fn ghs (convex: &Vec<VecDeque<Point>>) -> Option<(usize, usize, bool)> {
    // converting deque to vec
    let s0 = convex[0].as_slices();
    let mut p0 = s0.0.to_vec();
    p0.extend_from_slice(s0.1);

    let s1 = convex[1].as_slices();
    let mut p1 = s1.0.to_vec();
    p1.extend_from_slice(s1.1);

    let (left, right, first_is_left) = match (generate_u_l(&p0), generate_u_l(&p1)) {
        // .0 is the lower chain and .1 is the upper
        | (p0, p1) if p0.1[0].x < p1.1[0].x => (p0, p1, true),
        | (p0, p1) => (p1, p0, false),
    };

    let mut base = 0usize;
    let mut size = left.1.len();
    let mut candidate: usize = 0;
    while size > 1{
        let half = size / 2;
        let mid = base + half;
        let t_right = tangent_from_point(left.1[ mid ] , &right.0, &right.1, first_is_left);
        if let Some(t) = t_right {
            //t.1 is usize of 
            //println! ("t.1: {}", t.1.len());
            candidate = t.1;
            match is_tangent(mid , right.0[candidate], &left.0, false) {
                | Ordering::Equal => {base = mid; break;}
                | Ordering::Less => {}
                | Ordering::Greater => {base = mid}
            }
        } 
         size -= half;
    }
    if is_tangent(base, right.0[candidate] , &left.0, true ) == Ordering::Equal { Some((base, candidate, first_is_left))} else {None}
}

/*
pub fn naive_tangent (convex: &[[Point]]) {
    let (l1, u1) = generate_U_L(&convex[0]);
    let (l2, u2) = generate_U_L(&convex[1]);
    let mut prev_point = Point{x: 0.0, y: 0.0};
    if l1 > l2 {

    }
    let mut current_point = u2[u2.len()-1];
    n += 1;
    while prev_point.x == current_point.x && prev_point.y == current_point.y {
        if n % 2 == 1 {
            let value = tangent_from_point(current_point, &u1, &l1, true).unwrap_or((true, usize::MAX));
            if value.1 == usize::MAX {
                println! ("tangent not found on poly2");
            } else {
            current_point = if value.0 { u1[value.1]} else { l1[value.1]}; }
        } else {
            let value = tangent_from_point(current_point, &u2, &l2, false).unwrap_or((true, usize::MAX));
            if value.1 == usize::MAX {
                println! ("tangent not found on poly1");
            } else {
                current_point = if value.0 { u2[value.1]} else { l2[value.1]};
            }
        }
   }
}
*/

/*
pub fn naive_test() {
    let pi = std::f64::consts::PI;
    let l = 8;
    let mut p0: Vec<Point> = Vec::with_capacity(9);
    let mut p1: Vec<Point> = Vec::with_capacity(9);
    p0.push(Point{x: 1.0, y: 0.0});
    p1.push(Point{x: 1.8, y: 0.2});
    for i in 1..l+1 {
        p0.push(Point{x: ((pi*2.0*i as f64)/l as f64).cos(), y: ((pi*2.0*i as f64)/l as f64).sin()});
        p1.push(Point{x: ((pi*2.0*i as f64)/l as f64).cos()+0.8, y: ((pi*2.0*i as f64)/l as f64).sin()+0.2});
    }
    p0.push(Point{x: 1.0, y: 0.0});
    p1.push(Point{x: 1.8, y: 0.2});
    let tangents = naive_tangent_search(& vec!(p0, p1));
    for i in 0..2 {
        match tangents[i] {
            Some(t) => {println! ("tangents :{} {}", t.0, t.1);}
            None => {println! ("None");}
        }
    }
    /*for t in tangents {
        if let Some(res) = t {
            println! ("({}, {})", res.0, res.1);
        } else {
            println! ("None");
        }
    }*/
}
*/

#[test]
fn test_tangent_from_point () {
    let u1 = Point{x: 2.0, y: 0.0};
    let u2 = Point{x: 1.0, y: 2.0};
    let u3 = Point{x: 0.0, y: 2.5};
    let u4 = Point{x: -1.0, y: 2.0};
    let u5 = Point{x: -2.0, y: 0.0};

    let l1 = Point{x: -2.0, y: 0.0};
    let l2 = Point{x: -1.0, y: -2.0};
    let l3 = Point{x: 0.0, y: -2.5};
    let l4 = Point{x: 1.0, y: -2.0};
    let l5 = Point{x: 2.0, y: 0.0};

    let upper_chain = vec!(u1, u2, u3, u4, u5);
    let lower_chain = vec!(l1, l2, l3, l4, l5);
    let pi = std::f64::consts::PI;
    /*
    let point = Point{x: 8.0, y: 0.0};
    let res1 = binary_tangent_search(point, &upper_chain, true, true);
    if let Some(value1) = res1 {
    println! ("{}", value1); 
    } else {
        println! ("None");
    }
    */
    
    for i in 0..50 {
        // cirkel with radius 10
        let x = Point{x: 6.0*((2.0*pi*i as f64)/49.0).cos(), y: 6.0*((2.0*pi*i as f64)/49.0).sin()};
        let res = (tangent_from_point(x, &upper_chain, &lower_chain, true), tangent_from_point(x, &upper_chain, &lower_chain, false));
        if let (Some(value0), Some(value1)) = res {
            println! ("right tangent {}, left tangent {}.", value0.1 , value1.1,);
        } else {
            println! ("None");
        }
    }
}
#[test]
fn test_binary_search () {
    //lover
    //first test
    let c1 = Point {x: 0.0, y: 0.0};
    let c2 = Point {x: 1.0, y: 0.0};
    let c3 = Point {x: 2.0, y: 0.0};
    let c4 = Point {x: 3.0, y: 0.0};
    let c5 = Point {x: 4.0, y: 0.0};
    let c6 = Point {x: 5.0, y: 0.0};
    let c7 = Point {x: 6.0, y: 0.0};
    let chain1 = vec!(c1, c3);
    let someres = binary_search(c2, &chain1, false);
    assert_eq!(someres.is_some(), true);
    let res = someres.expect("First test failed in test_binary_search");
    assert_eq!(res, 0);
    //sec test 
    let chain2 = vec!(c1, c2, c4);
    let someres = binary_search(c3, &chain2, false);
    assert_eq!(someres.is_some(), true );
    let res = someres.expect("Second test failed in test_binary_search");
    assert_eq!(res, 1);
    //third
    let chain3 = vec!(c1, c3, c4);
    let someres = binary_search(c2, &chain3, false);
    assert_eq!(someres.is_some(), true) ;
    let res = someres.expect("thrid test failed in test_binary_search");
    assert_eq!(res, 0);
    //fourth
    let chain4 = vec!(c1, c2, c3, c4, c6);
    let someres = binary_search(c5, &chain4, false);
    assert_eq!(someres.is_some(), true );
    let res = someres.expect("fourth test failed in test_binary_search");
    assert_eq!(res, 3);

    let chain4 = vec!(c1, c3, c4, c5, c6, c7);
    let someres = binary_search(c2, &chain4, false);
    assert_eq!(someres.is_some(), true );
    let res = someres.expect("fith test failed in test_binary_search");
    assert_eq!(res, 0);


    //negative test
    let chain4 = vec!(c1, c2, c3, c4, c5, c6);
    let someres = binary_search(c7, &chain4, false);
    assert_eq!(someres.is_some(), false );
    

    //upper

    let chain1 = vec!(c3, c1);
    let someres = binary_search(c2, &chain1, true);
    assert_eq!(someres.is_some(), true);
    let res = someres.expect("First upper test failed in test_binary_search");
    assert_eq!(res, 0);
    //sec test 
    let chain2 = vec!(c4, c2, c1);
    let someres = binary_search(c3, &chain2, true);
    assert_eq!(someres.is_some(), true );
    let res = someres.expect("Second upper test failed in test_binary_search");
    assert_eq!(res, 0);
    //third
    let chain3 = vec!(c4, c3, c1);
    let someres = binary_search(c2, &chain3, true);
    assert_eq!(someres.is_some(), true );
    let res = someres.expect("thrid upper test failed in test_binary_search");
    assert_eq!(res, 1);
    //fourth
    let chain4 = vec!(c6, c4, c3, c2, c1);
    let someres = binary_search(c5, &chain4, true);
    assert_eq!(someres.is_some(), true );
    let res = someres.expect("fourth upper test failed in test_binary_search");
    assert_eq!(res, 0);

    let chain4 = vec!(c7, c6, c5, c4, c3, c1);
    let someres = binary_search(c2, &chain4, true);
    assert_eq!(someres.is_some(), true );
    let res = someres.expect("feth uppper test failed in test_binary_search");
    assert_eq!(res, 4);
}

