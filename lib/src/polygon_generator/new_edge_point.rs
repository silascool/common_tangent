use rand::distributions::{Distribution, Standard};
use rand::Rng;
use super::util::{Point, Polygon, linecross};



fn newpoint(mut rng: rand::prelude::ThreadRng) -> Point {
    let mut point: Point = rng.gen();
    point.x = (0.5-point.x)*200.0;
    point.y = (0.5-point.y)*200.0;
    return point
}

fn newpolygon() -> Polygon {
    Polygon{
        Points: Vec::new(),
        Edges: Vec::new(),
    }
}

/// I choose to 
pub fn random_select_polygon (n: usize) -> (Polygon, Polygon) {
    //Create a point list and a edge list for each of the polygons
    // put the first line into both polygons
    let poly0 = newpolygon();
    let poly1 = newpolygon();
    let mut polys = vec![poly0,poly1];
    let mut rng = rand::thread_rng();
    let x: Point = newpoint(rng);
    let y: Point = newpoint(rng);
    polys[0].Points.push(x);
    polys[0].Points.push(y);
    polys[0].Edges.push((0,1));
    let mut end = false;
    while !end {
        let x: Point = newpoint(rng);
        let y: Point = newpoint(rng);

        if !linecross(polys[0].Points[0], polys[0].Points[1], x, y) {
            polys[1].Points.push(x);
            polys[1].Points.push(y);
            polys[1].Edges.push((0,1));
            end = true;
        }
    }
    // put the rest of the lines into the polygons
    let mut u = 0;
    let mut lenght: usize = 2;
    while lenght < n {
        let mut found = false;
        while !found {
            found = true;
            let x: Point = newpoint(rng);
            let rn = rng.gen_range(0, lenght-1) as usize;
            let selectedpoint = polys[u].Points[rn];
                for j in &polys[0].Edges {
                    if linecross(polys[0].Points[j.0], polys[0].Points[j.1], selectedpoint, x) {
                        found = false;
                    }
                }
                for j in &polys[1].Edges {
                    if linecross(polys[1].Points[j.0], polys[1].Points[j.1], selectedpoint, x) {
                        found = false;
                    }
                }
            if found {
                polys[u].Points.push(x);
                polys[u].Edges.push((rn, lenght))
                }
            }
        u = 1 - u;
        lenght = polys[u].Points.len();
    }
    return (polys.pop().unwrap(), polys.pop().unwrap())
}
