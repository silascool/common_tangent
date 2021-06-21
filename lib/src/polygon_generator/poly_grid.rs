use super::grid::{Grid, Pointbox};
use super::util::{linecross, Point, Polygon};
use rand::Rng;

///----------------------------------------------------------------------------
///				Polygon genereation using grid and Pointbox
///----------------------------------------------------------------------------
fn first_points(mut rng: rand::prelude::ThreadRng) -> Point {
    let mut point: Point = rng.gen();
    point.x = point.x * 100.0;
    point.y = point.y * 100.0;
    return point;
}

fn new_point(pbox_pos: (i32, i32), mut rng: rand::prelude::ThreadRng) -> Point {
    let mut point: Point = rng.gen();
    point.x = (pbox_pos.0 as f64 + 0.5 + point.x/2.0) * 100.0;
    point.y = (pbox_pos.1 as f64 + 0.5 + point.y/2.0) * 100.0;
    return point;
}

fn gen_next_point(polynum: usize, grid: &mut Grid, 
    mut rng: rand::prelude::ThreadRng, n: usize) -> bool {
    
    // pick a alive box that also includes a point from polynum
    let alive_vec_pos = grid.alive_and_poly(polynum);
    let (picked_point, start_box_pos, from_box_index) =
        // If no box is alive we select     
        if alive_vec_pos.len()==0 {
            let picked_point_index = rng.gen_range(0, grid.points[polynum].len());
            let picked_point = grid.points[polynum][picked_point_index]; 
            let coordinates: (i32, i32) = 
                ((picked_point.x/ 100.0).floor() as i32, (picked_point.y/ 100.0).floor() as i32);        
            
            (picked_point_index
            , coordinates
            , grid.hmap.get(&coordinates).unwrap())

        } else {
            let start_box_pos = alive_vec_pos[rng.gen_range(0, alive_vec_pos.len())];
            let start_box_index = grid.alive.get(&start_box_pos).unwrap();
            let start_box = &grid.boxes[*start_box_index];
            // select point in the box where the new edge starts
            let box_points_len = start_box.points[polynum].len();
            
            (start_box.points[polynum][rng.gen_range(0, box_points_len)]
            , start_box.pos
            , start_box_index)
        };
    //println!("picked_point {:?}", picked_point);
    // find not dead neighboor boxes
    let candidates: &mut Vec<(i32, i32)> = &mut grid.alive_neighbours(start_box_pos); 
    if n == 1000 {
        for (key, _value) in &grid.alive {
            println!("{:?}", key);
        }
    }
    if candidates.len() == 0 {
        grid.alive.remove(&start_box_pos);
        return false
    }
    let destination_box = candidates[rng.gen_range(0, candidates.len())];
    let next_point = new_point(destination_box, rng);
    return grid.can_add_point(next_point, picked_point, *from_box_index, polynum) 
}

pub fn generate_poly(num_of_poly: usize, polysize: usize, limit: usize) 
    -> Vec<Polygon> {
    let mut grid = Grid::new(limit, num_of_poly);
    let mut rng = rand::thread_rng();
    
    //generate first point of each poly
    grid.push(Pointbox::new((0,0), num_of_poly));
    for i in 0..num_of_poly {
        grid.points.push(vec![first_points(rng)]);
        grid.boxes[0].points[i].push(0);
    }
    //println!("grid.points: {:?}", grid.points);
    //generate the rest of the polygon
    let mut iterations = 0;
    for i in 1..polysize {
        for polynum in 0..num_of_poly {
            while ! gen_next_point(polynum, &mut grid, rng, iterations) {
                iterations += 1;
            }
            println! ("{}: {}", i, iterations);
            iterations = 0;
        }
    }
    //for b in &grid.boxes {
    //    println!("b.pos: {:?}", b.pos);
    //}
    //convert the grid into Vec<Polygon>
    return grid.into_polys(num_of_poly);
} 


#[test]
fn gen_test() {
    let mut p_vec = generate_poly(2, 1000, 2);
    //assert_eq!(true, grid.can_add_point( Point{x: 1.0, y: 1.0}, 0, 0));
    assert_eq!(p_vec.len(), 2);
    assert_eq!(p_vec[0].points.len(), 1000);
}

#[test]
fn test_cross() {
    let mut p_vec = generate_poly(2, 10, 3);
    //assert_eq!(true, grid.can_add_point( Point{x: 1.0, y: 1.0}, 0, 0));
    assert_eq!(p_vec.len(), 2);
    assert_eq!(p_vec[0].points.len(), 10);
    let mut count: usize = 0;
    for i in &p_vec[0].edges {
        for j in &p_vec[1].edges {
            if linecross(p_vec[1].points[j.0],  p_vec[1].points[j.1], p_vec[0].points[i.0], p_vec[0].points[i.1]) {
                count = count + 1;
            }
        }
    } 
    assert_eq!(0,count);
}
