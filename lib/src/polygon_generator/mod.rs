use std::vec;
mod katapiller;
mod random_polygon;
mod random_polygonbox;
mod poly_grid;
mod grid;
use super::util::{Point, Polygon};
use super::util;




fn count_point_orcurences (pol: &Polygon) -> Vec<Vec<(usize,usize)>>{
    let len = pol.points.len();
    let mut edges_on_point: Vec<Vec<(usize,usize)>> = Vec::new();
    let edges = pol.edges.to_vec(); 
    for _i in 0..len {
        edges_on_point.push(Vec::new());
    }
    for edge in edges {
        edges_on_point[edge.0].push((edge.0, edge.1));
        edges_on_point[edge.1].push((edge.1, edge.0));
    }
    return edges_on_point
}

fn generate_weakly_simple (pol: &Polygon) -> Vec<Point> {
    //every point contains a vector of every edge that is connected to it (every edge is)
    let mut edges_on_point = count_point_orcurences(pol);
    let mut point_polygon: Vec<Point> = Vec::new();
    let mut lastedge = edges_on_point[0].pop().unwrap();
    let x = pol.points[lastedge.0];
    let y = pol.points[lastedge.1];
    point_polygon.push(x);
    point_polygon.push(y);
    let mut finished = false;
    while ! finished {
        let current_point = lastedge.1;
        // the cardinality of the point (node)
        let current_len = edges_on_point[current_point].len();
        match current_len {
            0 => {finished = true},
            _ => {
                // initializes the nextedge as return edge
                let mut next_point = 0;
                // notices that angles is impossibly large
                let mut min_angle: f64 = 10.0;
                // finds edge with smallest angle to lastedge
                for i in 0..current_len {
                    let p = edges_on_point[current_point][i];
                    let a = pol.points[lastedge.0];
                    let b = pol.points[lastedge.1];
                    //assert_eq!(b, pol.Points[p.0]);
                    let c = pol.points[p.1];
                    let p_angle = util::angle_between_3_points(a, b, c);
                    if p_angle < min_angle {
                        min_angle = p_angle;
                        next_point = i;
                    }
                }
                lastedge = edges_on_point[current_point].remove(next_point);
                point_polygon.push(pol.points[lastedge.1]);
            }
        }
    }
    return point_polygon
}


#[inline]
pub fn gen_polygon (n: usize) -> Vec<Vec<Point>>{
     let p = random_polygon::random_select_polygon(n);
    return vec!(generate_weakly_simple(&p.0), generate_weakly_simple(&p.1))
}

pub fn gen_boxpoly  (n: usize) -> Vec<Vec<Point>>{
    
    let p = random_polygonbox::random_select_polygon(n);
   return vec!(generate_weakly_simple(&p.0), generate_weakly_simple(&p.1))
}
pub fn gen_catapiller (n: usize) -> Vec<Vec<Vec<Point>>> {
    katapiller::katapiller_polygon(n)
}

pub fn gen_grid_poly (n: usize) -> Vec<Vec<Point>> {
    let p = poly_grid::generate_poly(2, n, 3);
    return vec!(generate_weakly_simple(&p[0]), generate_weakly_simple(&p[1]))

}

pub fn _linecross_tjekker(poly1: Vec<Point>, poly2: Vec<Point>) -> Vec<Vec<Point>> {
    let len1 = poly1.len();
    let len2 = poly2.len();
    let mut retvec = Vec::new();
    let mut _tjek = true;
    for i in 1..len1-1 {
        for j in 1..len2-1{
            if util::linecross(poly1[i-1], poly1[i], poly2[j-1], poly2[j]){
                retvec.push(vec![poly1[i-1], poly1[i], poly2[j-1], poly2[j]]);
                _tjek = false;
            }
        }
    }
    for i in 1..len2-1 {
        for j in 1..len1-1{
            if util::linecross(poly2[i-1], poly2[i], poly1[j-1], poly1[j]){
                retvec.push(vec![poly2[i-1], poly2[i], poly1[j-1], poly1[j]]);
                _tjek = false;
            }
        }
    }
    return retvec
}



#[test]
fn test_weakly_simple() {
    let a = Point{x: 0.0, y: 0.0};
    let b = Point{x: 1.0, y: 0.0};
    let c = Point{x: 1.0, y: 1.0};
    let d = Point{x: 0.0, y: 1.0};
    let e = Point{x: 1.0, y: 2.0};
    let f = Point{x: 2.0, y: 1.0};
    let g = Point{x: 2.0, y: 2.0};
    let points = vec!(a, b, c, d, e, f, g);
    let e1 = (0,1);
    let e2 = (1,2);
    let e3 = (2,3);
    let e4 = (2,4);
    let e5 = (2,5);
    let e6 = (5,6);
    let edges = vec!(e1,e2,e3,e4,e5,e6);
    let p = Polygon{points: points, edges: edges};
    let weakly_p = generate_weakly_simple(&p);

    // tjek if the result of weakly has same lenght as abcfgfcecdcba

    assert_eq!(weakly_p.len(), 13);

    let abcfgfcecdcba = vec!(a,b,c,f,g,f,c,e,c,d,c,b,a);

    for i in 0..13 {
        print! ("({},{}) ", weakly_p[i].x, weakly_p[i].y);
        assert_eq! (abcfgfcecdcba[i].x, weakly_p[i].x);
        assert_eq! (abcfgfcecdcba[i].y, weakly_p[i].y);
    }
    println! ("");
}

#[test]
fn test_weakly_simple_large_n() {
    for i in 1..3 {
        let n = 10i64.pow(i);
        let (p, _q) = random_polygonbox::random_select_polygon(n as usize);
        let pweak = generate_weakly_simple(&p);
        let res = 2*n-1;
        assert_eq!(pweak.len(), res as usize);
    }
}

#[test]
fn test_grid_to_weakly_simple() {
    for i in 1..5 {
        let n = 10i64.pow(i);
        let poly_vec = poly_grid::generate_poly(2, n as usize, 3);
        let pweak = generate_weakly_simple(&poly_vec[0]);
        let res = 2*n-1;
        assert_eq!(pweak.len(), res as usize);
    }
}
