use serde::{Serialize, Deserialize};
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Polygon{
    pub points: Vec<Point>,
    pub edges: Vec<(usize,usize)>,
}

pub fn modular (val: i64, n: i64) -> usize {
    match val {
        | x if x < 0 && x.abs() > n =>  modular(val+n, n),
        | x if x < 0 =>                 (n - val.abs()) as usize,
        | _ =>                          (val % n) as usize
    }
}

fn det(p: Point, q: Point) -> f64 {
    p.x*q.y-p.y*q.x
}

pub fn plane(a: Point, b: Point, c: Point) -> bool {
    det(a, b) + det(b, c) + det(c, a) <= 0.0
}
pub fn is_right_of(a: Point, b: Point, c: Point) -> bool {
    det(a, b) + det(b, c) + det(c, a) <= 0.0
}
pub fn is_left_of(a: Point, b: Point, c: Point) -> bool {
    det(a, b) + det(b, c) + det(c, a) >= 0.0
}
pub fn right_colinear_left(a: Point, b: Point, c: Point) -> i8 {
    match det(a, b) + det(b, c) + det(c, a) {
        x if x < 0.0 => 1,
        x if x > 0.0 => -1,
        _ => 0,
    }
}

pub fn det_star(a: Point, b: Point, c: Point) -> f64 {
    det(a, b) + det(b, c) + det(c, a)
}
/// function returns true if the x point is in the triangle ABC
/// 
/// 
/// 
/// 
pub fn in_triangle(x: Point, a: Point, b: Point, c: Point) -> bool {
    ((det_star(a, b, x)>= 0.0) && (det_star(b, c, x)>= 0.0)) && (det_star(c, a, x)>=0.0)
}


pub fn find_left_n_right_xtreme (p: &Vec<Point>) -> (usize, usize) {
    let mut h = 0;
    let mut l = 0;
    for i in 1..p.len()-1 {
        if p[h].x < p[i].x {
            h = i;
        }
        if p[l].x > p[i].x {
            l = i;
        }
    }
    return (l, h)
}

pub fn linecross (p1: Point, p2: Point, q1: Point, q2: Point) -> bool {
    if (det_star(p1, p2, q2) > 0.0) == (det_star(p2, p1, q1) > 0.0) && // 
    (det_star(q1, q2, p1) > 0.0) == (det_star(q2, q1, p2) > 0.0) {
        true
    } else {
        false
    }
}

pub fn angle_between_3_points(a: Point, middel: Point, c: Point) -> f64 {
    let result = (c.y-middel.y).atan2(c.x - middel.x) - (a.y-middel.y).atan2(a.x - middel.x);
    if result <= 0f64 {
        return 2.0*std::f64::consts::PI + result
    } else {
        return result
    }
}

pub fn test() {
    let pi = std::f64::consts::PI;
    let l = 10;
    for i in 0..l+1 {
        for j in 0..l+1 {
            let a = Point{x: ((pi*2.0*i as f64)/l as f64).cos(), y: ((pi*2.0*i as f64)/l as f64).sin()};
            let origo = Point{x: 0.0, y: 0.0};
            let c = Point{x: ((pi*2.0*j as f64)/l as f64).cos(), y: ((pi*2.0*j as f64)/l as f64).sin()};
            let angle = angle_between_3_points(a, origo, c);
            println! ("{} {} = {}", i, j, angle);
        }
    }
}
