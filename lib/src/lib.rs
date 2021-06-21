pub mod aw;
pub mod ghs;
pub mod util;
pub mod polygon_generator;
pub mod melkmanns;
pub mod json;

fn melk_to_ghs (polys: &Vec<Vec<util::Point>>) -> Option<(usize,usize, bool)>{
    let v = vec!(melkmanns::algorithm_hull(&polys[0]), melkmanns::algorithm_hull(&polys[1]));
    println!("v_len(): {}", v[0].len());
    ghs::ghs(&v)
}

#[test]
fn Aw_eq_ghs(){
    for i in  [10, 50, 100, 200, 500].iter() {
        let polys = polygon_generator::gen_polygon(*i);
        let aw_t = aw::algorithm([-1i8, -1i8], &polys).unwrap();
        let ghs_t = melk_to_ghs(&polys).unwrap_or((0,0,false));
    }
    assert!(false);
}
