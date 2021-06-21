use super::util::{Point, linecross};
use rand::Rng;

pub fn katapiller_polygon (n: usize) -> Vec<Vec<Vec<Point>>> {
    //Generating the points
    let mut rng = rand::thread_rng();
    let mut points = Vec::new();
    for _i in 0..n {
        let mut point: Point = rng.gen();
        point.x = (0.5-point.x)*200.0;
        point.y = (0.5-point.y)*200.0;
        points.push(point);
    }
    // initialize the polygons as Vec of Points.
    let mut polygon0 = Vec::new();
    let mut polygon1 = Vec::new();
    // generating the first line of polygon0
    polygon0.push(vec![ points.pop().unwrap(),  points.pop().unwrap()]);

    // generating the first line of polygon1
    let p = points.pop().unwrap();
    let mut tjek = true; 
    let mut i = 0;
    while i < points.len() && tjek {
        if !linecross(p, points[i], polygon0[0][0], polygon0[0][1]) {
            //println!("something");
            tjek = false;
            let q = points.remove(i);
            polygon1.push(vec![p, q]);
        }   
        i = i + 1;
    }  
    let mut polygons = vec![polygon0, polygon1];
    let mut u = 0;
    let mut y = vec!(0,0);
    while let Some(point) = points.pop() {
        //lines.push(p);
        tjek = true;
        let mut end = false;
        let mut j = polygons[u].len();
        while j>0 && !end {
            // tjek cross with other polygon
            for e in &polygons[1-u] {
                if linecross(point, polygons[u][j-1][y[u]], e[0], e[1]){
                    tjek = false;
                    //println! ("{} was false", u);
                }
            }
            // tjek cross with it self
            for e in &polygons[u] {
                if linecross(point, polygons[u][j-1][y[u]], e[0], e[1]) {
                    //print! ("{} was false", u);
                    tjek = false
                } 
            }
            //println! ("{} {} {}", tjek, u, j);
            if tjek {
                let q = polygons[u][j-1][y[u]];
                polygons[u].push(vec![point, q]);
                end = true;
            } else {
                j -= 1;
            }
        }

        y[u] = 1 - y[u];
        u = 1 - u;
        //println! ("{}", u);
    }
    return polygons
}