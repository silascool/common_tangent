use super::util::{linecross, Point, Polygon};
use std::collections::HashMap;

///-----------------------------TODO-------------------------------------------
///            
/// make a function that generates polygon type from grid.
///
///---------------------------------------------------------------------------

pub struct Pointbox {
    pub pos: (i32, i32),
    //first value in edges is with
    in_edges: Vec<Vec<(usize, usize)>>,
    out_edges: Vec<Vec<(usize, usize)>>,
    pub points: Vec<Vec<usize>>,
}



impl Pointbox {
    pub fn new (pos: (i32,i32), num_of_poly: usize) -> Self {
        let mut in_edges = Vec::new();
        let mut out_edges = Vec::new();
        let mut points = Vec::new();
        (0..num_of_poly).for_each(|_i| {
            in_edges.push(vec![]);
            out_edges.push(vec![]);
            points.push(vec![]);
        });
        //println!("lenght of edges {}", edges.len());
        Self {
            pos: pos,
            in_edges: in_edges,
            out_edges: out_edges,
            points: points,
        }
    }

    fn get_in_edges(&self) -> Vec<(u8, &(usize, usize))> {
        let mut poly: u8 = 0;
        self.in_edges
            .iter()
            .map(|v| {
                poly += 1;
                v.iter()
                    .map(|u| (poly -1, u))
                    .collect::<Vec<(u8, &(usize, usize))>>()
            })
            .flatten()
            .collect()
    }

    fn get_all_edges(&self) -> Vec<(u8, &(usize, usize))> {
        let mut poly: u8 = 0;
        let mut a_edges :  Vec<(u8, &(usize, usize))>= self.in_edges
            .iter()
            .map(|v| {
                poly += 1;
                v.iter()
                    .map(|u| (poly -1, u))
                    .collect::<Vec<(u8, &(usize, usize))>>()
            })
            .flatten()
            .collect();
        let mut poly: u8 = 0;
        let mut o_edges : Vec<(u8, &(usize, usize))> = self.out_edges
            .iter()
            .map(|v| {
                poly += 1;
                v.iter()
                    .map(|u| (poly -1, u))
                    .collect::<Vec<(u8, &(usize, usize))>>()
            })
            .flatten()
            .collect();
        a_edges.append(&mut o_edges);
        return a_edges
    }


    fn contains_elements_of_poly (&self, poly: usize) -> bool {
        ! self.points[poly].is_empty()
    }
    fn len(&self) -> usize {
        self.points.iter().fold(0, |acc, vec| acc + vec.len())
    }
    fn consume(self) -> Vec<Vec<(usize, usize)>> {
        self.in_edges
    }
}

/// The problems with previous polygon generation is that each time we add a new
/// edge, we have to check all the existing edges.
/// The new structure that inplements neighbor regions such that we only have to
/// check the edges in the neighborhood.
/// 
/// boxes: the neighborhood we divide the plane into.
/// alive: is a hashmap that has a reference to wich boxes still have space for more point. the
/// tuple is (poly, pos)
/// hmap: given a koordinate gives the index in the boxes vec.
/// limit: 
/// num_of_poly: number of polygons to create
pub struct Grid {
    pub boxes: Vec<Pointbox>,
    pub alive: HashMap<(i32, i32), usize>,
    pub hmap: HashMap<(i32,i32), usize>,
    pub points: Vec<Vec<Point>>,
    capacity: usize,
    num_of_poly: usize,
}

impl Grid { 
    /// Construcktor for Grid
    pub fn new(capacity: usize, num_of_poly: usize) -> Self {
        Self {
            boxes: Vec::new(),
            alive: HashMap::new(),
            hmap: HashMap::new(),
            points: Vec::new(),
            capacity: capacity,
            num_of_poly: num_of_poly,
        }
    }

    // adds new box to grid, and updates hashmap and alivemap
    pub fn push(&mut self, pbox: Pointbox) -> &mut Self {
        self.hmap.insert(pbox.pos, self.boxes.len());
        self.alive.insert(pbox.pos, self.boxes.len());
        self.boxes.push(pbox);
        self
    }

    /// Figures out if a pointbox is alive or never constructed.
    fn is_dead(&self, coord: (i32, i32)) -> bool {
        match (self.hmap.get(&coord), self.alive.get(&coord)) {
            | (Some(_), None) => true,
            | _ => false,
        }
    }

    /// lists the alive Pointbox's that also include a point from a given 
    pub fn alive_and_poly(&self, poly: usize) -> Vec<(i32,i32)> {
        self.alive.iter()
            .map(|(_, i)| &self.boxes[*i])
            .filter(|b| b.contains_elements_of_poly(poly))
            .map(|b| b.pos)
            .collect::<Vec<_>>()
    }

    pub fn alive_neighbours(&self, coord: (i32, i32)) -> Vec<(i32, i32)> {
        let relative_neighbours = vec![
            (0, 0),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];
        relative_neighbours.into_iter()
            .filter(|n| !self.is_dead((n.0 + coord.0, n.1 + coord.1)))
            .collect::<Vec<_>>()
    }

    // note that neighbours include self
    fn neighbours(&self, coord: (i32, i32)) -> Vec<&usize> {
        let relative_neighbours = vec![
            (0, 0),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];
        relative_neighbours.into_iter()
            .filter_map(|n| self.hmap.get(&(&n.0 + coord.0, &n.1 + coord.1)))
            .collect::<Vec<&usize>>()
    }
   
    fn check_neighbours<F>(&self, coord: (i32, i32), f: F) -> bool  where
    F: Fn((u8, &(usize, usize))) -> bool {
        let edges = self.neighbours(coord).iter()
            .cloned()
            .flat_map(|i| self.boxes[*i].get_all_edges())
            .collect::<Vec<_>>();
        if edges.is_empty() {
            return true
        } else {
            // println! ("edges {:?}", edges);
            return !edges.iter().any(|u| f(*u))
        }
    }

    /// Checks wether a linesection (newp, oldp) cross's a linesection in the
    /// box or its neighbours.
    fn does_not_cross (&self, coord: (i32,i32), newp: Point, oldp_index: usize, poly: usize) -> bool {
        //println!("self.points: {:?}", self.points);
        //println!("oldp_index: {:?}", oldp_index);

        self.check_neighbours(coord,  |uu| {
            //linecross return true if a line crosses
                linecross(self.points[uu.0 as usize][uu.1.0], 
                    self.points[uu.0 as usize][uu.1.1],
                    self.points[poly as usize][oldp_index],
                    newp)
                })
    }

    /// used to limit amount of points each polygon has in a box
    fn is_boxed_filled(&self, pbox_index: &usize, poly: usize) -> bool {
        if self.boxes[*pbox_index].points.is_empty() {
            true 
        } else {
            self.boxes[*pbox_index].points[poly].len() < self.capacity
        }
    }
    
    fn add_point (&mut self, newp: Point, oldp_index: usize, pboxes: (usize, usize),
        poly: usize) -> &mut Self {
        //push newp to points of gird
        self.points[poly].push(newp);
        //push newp to points of pointbox
        self.boxes[pboxes.0].points[poly].push(self.points.len()-1);
        //push (oldp, newp) to in_edges of destination pointbox
        self.boxes[pboxes.0].in_edges[poly].push((oldp_index, self.points[poly].len()-1));
        //push (oldp, newp) to out_edges of from pointbox
        self.boxes[pboxes.1].out_edges[poly].push((oldp_index, self.points[poly].len()-1));

        if self.boxes[pboxes.0].points[poly].len() == self.capacity {
            self.alive.remove(&self.boxes[pboxes.0].pos);
        }
        self 
    }

    pub fn can_add_point (&mut self, newp: Point, oldp_index: usize, from_pbox: usize, poly: usize) -> bool {
        // find witch box newp is in
        let coordinates: (i32, i32) = 
            ((newp.x / 100.0).floor() as i32, (newp.y / 100.0).floor() as i32);        
        //println!("{},{}",coordinates.0, coordinates.1);
        match self.hmap.get(&coordinates) {
            | None if self.does_not_cross(coordinates, newp, oldp_index, poly) => {
                self.push(Pointbox::new(coordinates, self.num_of_poly));
                self.add_point(newp, oldp_index, (self.boxes.len() - 1, from_pbox) , poly);
                return true;
            }
            // if pbox with coordniates exist, isnt filed and our new line doesn't cross
            | Some(i) if self.does_not_cross(coordinates, newp, oldp_index, poly) => {
                    let dst_pbox_index = *i;
                    self.add_point(newp, oldp_index, (dst_pbox_index, from_pbox), poly);
                    return true;
            }
            | _ => return false,
                // Check line crosses
            }
    }

    /// Cosumes self and converts into the type Polygon
    pub fn into_polys(self, polynr: usize) -> Vec<Polygon> {
        // Collect all the edges in all the boxes
        let mut edges: Vec<Vec<(usize, usize)>> = Vec::new();
        let iter_b = self.boxes.into_iter();
        iter_b.for_each(|b| b.consume().into_iter().for_each(|e| edges.push(e)));
        //format the edges vec such that edges[0] is the edges of poly 0

        let ite = edges.into_iter();
        let mut p_edges: Vec<Vec<(usize, usize)>> = 
            (0..polynr).map(|i| {
                ite.clone()
                    .skip(i)
                    .step_by(polynr)
                    //.inspect(|x| println!("after skip: {:?}", x))
                    .fold(vec![], |mut vec, mut x| {
                        vec.append(&mut x);
                        vec
                    })  
            })
            .collect();
        //println!("hele p_edges: {:?}", p_edges);
        // returns vec of poly.
        // obs reverts the order of both points and edges such as poly[0].edges
        // equals p_edges.last
        let mut points = self.points;
        (0..polynr).map(|_i| Polygon {
            points: points.pop().unwrap(),
            edges: p_edges.pop().unwrap(),})
            .collect::<Vec<Polygon>>()
    }
}

#[test]
fn grid_into_polys() {
    let mut points: Vec<Vec<Point>> = Vec::new();
    points.push(vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 1.0 },
        Point { x: 2.0, y: 2.0 },
        Point { x: 3.0, y: 3.0 },
    ]);
    points.push(vec![
        Point { x: 4.0, y: 4.0 },
        Point { x: 5.0, y: 5.0 },
        Point { x: 6.0, y: 6.0 },
        Point { x: 7.0, y: 7.0 },
    ]);
    let mut grid = Grid {
        boxes: Vec::new(),
        alive: HashMap::new(),
        hmap: HashMap::new(),
        points: points,
        capacity: 3,
        num_of_poly: 2,
    };
    let mut a = Pointbox::new((0, 0), 2);
    a.in_edges.append(&mut vec![
        vec![(0, 1), (0, 2), (1, 3)],
        vec![(1, 1), (1, 2), (1, 3)],
    ]);
    let mut b = Pointbox::new((1, 1), 2);
    b.in_edges.append(&mut vec![
        vec![(2, 1), (2, 2), (2, 3)],
        vec![(3, 1), (3, 2), (0, 3)],
    ]);
    grid.push(a);
    grid.push(b);
    let polys = grid.into_polys(2);
    assert_eq!(polys.len(), 2);
    //since
    assert_eq!(
        polys[1].edges,
        vec![(0, 1), (0, 2), (1, 3), (2, 1), (2, 2), (2, 3)]
    );
    assert_eq!(polys[0].points.len(), 4);
}

#[test]
fn grid_can_cross() {
    let mut points: Vec<Vec<Point>> = Vec::new();
    let mut grid = Grid::new(4, 2); 
    grid.push(Pointbox::new((0,0), 2)); 
    grid.points.push(vec![Point {x: 1.0, y: 99.0}]);
    grid.points.push(vec![Point {x: 1.0, y: 1.0}]);
    grid.boxes[0].points[0].push(0);
    grid.boxes[0].points[1].push(0);
    assert_eq!(true, grid.does_not_cross((0,1), Point{x: 101.0, y: 1.0}, 0, 0));
    assert!(grid.can_add_point (Point{x: 101.0, y: 1.0}, 0, 0, 0));
    assert_eq!(2, grid.boxes.len());
    println! ("neighbours of (0,1) {:?}", grid.neighbours((0,1)));
    assert!(linecross(grid.points[0][0], grid.points[0][1],
                    grid.points[1][0], Point{x: 101.0, y: 99.0}));
    assert_eq!(false, grid.does_not_cross((0,1), Point{x: 101.0, y: 99.0}, 0, 1));
    assert!(!grid.can_add_point (Point{x: 101.0, y: 99.0}, 0, 0, 1));
    assert!(grid.can_add_point (Point{x: 1.0, y: -99.0}, 0, 0, 1));
    println! ("neighbours of (0,1) {:?}", grid.neighbours((0,0)));
}

/// Not needed because of hashmap. keeping it in here maybe i am wrong an will need it
///
///
///
/// This is from https://math.stackexchange.com/questions/163080/on-a-two-dimensional-grid-is-there-a-formula-i-can-use-to-spiral-coordinates-in
/// It maps the natural numbers to Z x Z in a spiral pattern.
/// And i didn't do a very nice job of porting it to rust

fn spiral(n: usize) -> (i32, i32) {
    let k = (((n as f64).sqrt() - 1.0) / 2.0).ceil() as i32;
    let mut t = 2 * k + 1;
    let mut m = t.pow(2);
    t = t - 1;
    let mut done = false;
    let mut res: (i32, i32) = (0, 0);
    if n as i32 >= m - t {
        res = (k - (m - n as i32), -k);
        done = true;
    } else {
        m = m - t;
    }
    if n as i32 >= m - t && !done {
        res = (-k as i32, (-k as i32) + (m - n as i32));
        done = true;
    } else {
        m = m - t;
    }
    if n as i32 >= m - t && !done {
        res = ((-k) + (m - n as i32), k);
    } else if !done {
        res = (k, (k - (m - n as i32 - t)));
    }
    return res;
}

///  17--16--15--14--13
///  |               |
///  18    5--4--3   12
///  |     |     |   |
///  19    6  1--2   11
///  |     |         |
///  20    7--8--9--10
///  |
///  21--22--23--24--25--26
///
/// zxz is the inverse funktion off spiral
/// it uses the that uneven squares are lokalted on 9 is on (1,-1), 25 is on (2,-2), 49 is (3,-3).
/// the same can be said off the even squares the are lokated on 4 is on (0,1), 16 is on (-1,2), 36 is on (-2, 3).
pub fn zxz_to_n(x: i32, y: i32) -> usize {
    if (x == 0) && (y == 0) {
        1
    } else {
        let max = if x.abs() >= y.abs() { x } else { y };
        match (x == max, max > 0) {
            //right column
            (true, true) => {
                if x == y.abs() && y < 0 {
                    (2 * max.abs() + 1).pow(2) as usize
                } else {
                    ((2 * max.abs() - 1).pow(2) + (max + y)) as usize
                }
            }
            //top row
            (false, true) => ((2 * max).pow(2) - (max - 1 + x)) as usize,
            //left column
            (true, false) => ((2 * max).pow(2) + (max.abs() - y + 1)) as usize,
            //bottom row
            (false, false) => ((2 * max.abs() + 1).pow(2) + (x - max.abs())) as usize,
        }
    }
}

#[test]
fn grid_neigbours() {
    let mut grid = Grid {
        boxes: Vec::new(),
        alive: HashMap::new(),
        hmap: HashMap::new(),
        points: Vec::new(),
        capacity: 3,
        num_of_poly: 2,
    };
    grid.push(Pointbox::new((0, 0), 2));
    grid.push(Pointbox::new((2, 2), 2));
    grid.push(Pointbox::new((-1, 0), 2));
    assert_eq!(grid.neighbours((0, 0)), vec![&0, &2]);
}

#[test]
fn grid_add_points() {
    let mut grid = Grid {
        boxes: Vec::new(),
        alive: HashMap::new(),
        hmap: HashMap::new(),
        points: Vec::new(),
        capacity: 3,
        num_of_poly: 2,
    };
    grid.push(Pointbox::new((0, 0), 2));
    grid.push(Pointbox::new((2, 2), 2));
    grid.push(Pointbox::new((-1, 0), 2));
    assert_eq!(grid.neighbours((0, 0)), vec![&0, &2]);
}

/*
#[test]
fn can_add_point_test(){
    let mut grid = Grid {
        boxes: Vec::new(),
        alive: HashMap::new(),
        hmap: HashMap::new(),
        points: Vec::new(),
        capacity: 3,
        num_of_poly: 2,
    };
    



}
*/
/// Tests for zxz_to_n and spiral

#[test]
fn top_zxz() {
    let fasit: Vec<usize> = vec![3, 4, 5];
    let input: Vec<(i32, i32)> = vec![(1, 1), (0, 1), (-1, 1)];
    let res: Vec<usize> = input.into_iter().map(|x| zxz_to_n(x.0, x.1)).collect();
    assert_eq!(fasit, res);
}

#[test]
fn zxz() {
    let input: Vec<(i32, i32)> = vec![
        (0, 0),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (2, -1),
        (2, 0),
        (2, 1),
        (2, 2),
        (1, 2),
        (0, 2),
        (-1, 2),
        (-2, 2),
        (-2, 1),
        (-2, 0),
        (-2, -1),
        (-2, -2),
        (-1, -2),
        (0, -2),
        (1, -2),
        (2, -2),
    ];
    let res: Vec<usize> = input.into_iter().map(|x| zxz_to_n(x.0, x.1)).collect();
    let fasit: Vec<usize> = (1..26).collect();
    assert_eq!(fasit, res);
}

#[test]
fn spiral_to_zxz() {
    let input: Vec<usize> = (1..10001).collect();
    let fasit: Vec<usize> = (1..10001).collect();
    let res: Vec<usize> = input
        .into_iter()
        .map(|x| {
            let s = spiral(x);
            zxz_to_n(s.0, s.1)
        })
        .collect();
    assert_eq!(fasit, res);
}
