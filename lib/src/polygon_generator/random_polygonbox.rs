//use rand::distributions::{Distribution, Standard};
use rand::Rng;
use super::util::{Point, Polygon, linecross};

/*
impl Distribution<Point> for StandardNormal {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let rand_x = self.sample(rng);
        let rand_y = self.sample(rng);
        Point {
            x: rand_x,
            y: rand_y,
use rand::Rng;
        }
    }
}
fn newpolygon() -> Polygon {
    Polygon{
        Points: Vec::new(),
        Edges: Vec::new(),
    }
}
fn firstpoint(mut newpoint: Point) -> Point {
    newpoint.x = 0.5-newpoint.x*10.0;
    newpoint.y = 0.5-newpoint.y*10.0;
    return newpoint
}


fn newpoint(point: Point, mut newpoint: Point) -> Point {
    newpoint.x = point.x+(0.5-newpoint.x)*10.0;
    newpoint.y = point.x+ (0.5-newpoint.y)*10.0;
    return newpoint
}

/// I choose to 
pub fn random_select_polygon (n: usize) -> (Polygon, Polygon) {
    //Create a point list and a edge list for each of the polygons
    // put the first line into both polygons
    let poly0 = newpolygon();
    let poly1 = newpolygon();
    let mut polys = vec![poly0,poly1];
    let mut rng = SmallRng::from_seed([12;16]);
    //from_entropy();
    let a0: Point = firstpoint(StandardNormal.sample(&mut rng));

*/

fn newpoint(mut rng: rand::prelude::ThreadRng) -> Point {
    let mut point: Point = rng.gen();
    point.x = (0.5-point.x)*200.0;
    point.y = (0.5-point.y)*200.0;
    return point
}

fn newpolygon() -> Polygon {
    Polygon{
        points: Vec::new(),
        edges: Vec::new(),
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
    polys[0].points.push(x);
    polys[0].points.push(y);
    polys[0].edges.push((0,1));
    let mut end = false;
    while !end {
        let x: Point = newpoint(rng);
        let y: Point = newpoint(rng);

        if !linecross(polys[0].points[0], polys[0].points[1], x, y) {
            polys[1].points.push(x);
            polys[1].points.push(y);
            polys[1].edges.push((0,1));
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
            let selectedpoint = polys[u].points[rn];
                for j in &polys[0].edges {
                    if linecross(polys[0].points[j.0], polys[0].points[j.1], selectedpoint, x) {
                        found = false;
                    }
                }
                for j in &polys[1].edges {
                    if linecross(polys[1].points[j.0], polys[1].points[j.1], selectedpoint, x) {
                        found = false;
                    }
                }
            if found {
                polys[u].points.push(x);
                polys[u].edges.push((rn, lenght))
                }
            }
        u = 1 - u;
        lenght = polys[u].points.len();
    }
    return (polys.pop().unwrap(), polys.pop().unwrap())
}
