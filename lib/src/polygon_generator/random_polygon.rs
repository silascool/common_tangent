use rand::distributions::{Distribution, Standard};
use rand::{SeedableRng, Rng, rngs::SmallRng};
//use rand_distr::StandardNormal;
use super::util::{Point, Polygon, linecross};

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}
fn newpolygon() -> Polygon {
    Polygon{
        points: Vec::new(),
        edges: Vec::new(),
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
    //let mut rng = SmallRng::from_seed([12;16]);
    let mut rng = SmallRng::from_seed([14;16]);
    //from_entropy();
    let a0: Point = firstpoint(rng.gen::<Point>());
    let b0: Point = newpoint(a0, rng.gen::<Point>());
    polys[0].points.push(a0);
    polys[0].points.push(b0); 
    polys[0].edges.push((0,1));

    let mut end = false;

    let c0: Point = firstpoint(rng.gen::<Point>());
    while !end {
        let d0: Point = newpoint(a0, rng.gen::<Point>());
        if !linecross(polys[0].points[0], polys[0].points[1], c0, d0) {
            polys[1].points.push(c0);
            polys[1].points.push(d0);
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
            let rn = rng.gen_range(0, lenght-1) as usize;
            let selectedpoint = polys[u].points[rn];
            let x: Point = newpoint(selectedpoint, rng.gen::<Point>());
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
