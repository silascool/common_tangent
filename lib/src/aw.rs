use super::util::{Point, det_star, in_triangle, modular}; 

/// The algorithm from "Common Tangents of Two Disjoint Polygons in Linear Timeand Constant Workspace" M. Abrahamsen and B. Walczak
/// https://arxiv.org/abs/1601.01816
/// 
/// Returns two point that reprecents a common tangent of the two polygons.   
/// 
/// Input
/// alpha_0, alpha_1: {-1, 1}              decides witch way the array's are traversed
/// p[0] and P[1]: the arrays that contain the points of the two polygons
/// rigth now we assume that the polygons are orietated clockwise
/// 
pub fn algorithm(alpha: [i8;2], p: &Vec<Vec<Point>>) -> Option<(usize, usize)> {
    let n = [p[0].len() as i64, p[1].len() as i64];
  
    let (mut s , i) : ([i64;2], [i8;2]) = match (alpha[0], alpha[1]) {
            | (1, 1)    =>    ([n[0]-1 , 0],      [-1, 1]),
            | (-1, -1)  =>    ([0, n[1]-1],       [1, -1]),
            | (1, -1)   =>    ([0, 0],            [1, 1]),
            | (-1, 1)   =>    ([n[0]-1, n[1]-1],  [-1, -1]),
            | _         =>    ([0, n[1]-1],       [1, -1]),
    }; 
    let mut v = s.to_vec();
    let mut b = [false,false]; 
    let mut u = 0;
    while s[0].abs() < 2*n[0] &&
        (s[1].abs() ) < 2*n[1] &&
        (v[0].abs() < s[0].abs() + n[0] || v[1].abs() < s[1].abs()  + n[1]) {
            // v_u = v_u + 1
            v[u] = v[u] + i[u] as i64;
            // if p_u[v_u] not ∈ H_u(p_0[s_0],p_1[s_1]) and not b_u
            if !(alpha[u] as f64 * det_star(p[0][modular(s[0],n[0])], p[1][modular(s[1], n[1])], p[u][modular(v[u], n[u])]) <= 0.0) && !b[u] {
                // p_(1−u)[s_(1−u)]∈∆(p_u[s_u],p_u[v_u−1],p_u[v_u])
                if in_triangle(p[1 - u][modular(s[1-u], n[1-u])], p[u][modular(s[u], n[u])], p[u][modular(v[u]-1, n[u])], p[u][modular(v[u], n[u])]) {
                    b[u] = true;
                } else {
                    s[u] = v[u];
                    v[1-u] = s[1-u];
                    b[1-u] = false;
                }
            }
            u = 1 - u;
        }
    if s[0].abs()  >= 2*n[0] || s[1].abs()  >= 2*n[1] || b[0] || b[1] {
        return None
    }
    return Some((modular(s[0],n[0])  as usize, modular(s[1],n[1]) as usize))
}
pub fn are_disjoint (p: &Vec<Vec<Point>>) -> usize {
    let mut alphas = vec!([1,1], [-1,1], [1, -1], [-1, 1]);
    let mut count = 0;
    while let Some(a) = alphas.pop() {
        if algorithm(a, p).is_some(){
            count = count + 1;
        } 
    }
    return count;
}
#[test]
fn test () {
    println! ("hej");
}
