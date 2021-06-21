use piston_window::*;
use libary::util::{Point, plane, right_colinear_left};
use std::collections::VecDeque;
use libary::melkmanns;
use libary::ghs::*;
use libary::aw::algorithm;
use libary::util::{det_star, modular, in_triangle};
use libary::polygon_generator;

pub fn draw_deque (polys: &Vec<Vec<Point>>) {
    
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new(
        "Convexhull",
        [600, 600]
    )
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_lazy(true);
    let mut size = 0.0f64;
    let mut poly1 : VecDeque<Point> = VecDeque::new();
    let mut n = 2usize;
    let v1 = polys[0][0]; let v2 = polys[0][1]; let v3 = polys[0][2];
    if plane (v1, v2, v3) {
        poly1.push_back(v1);
        poly1.push_back(v2); 
    } else {
        poly1.push_back(v2);
        poly1.push_back(v1);
    }
    poly1.push_back(v3);
    poly1.push_front(v3);
    
    if polys[0].len() > 2 && polys[1].len() > 2 && n < polys[0].len() -1 {
        for p in &polys[0] {
            if p.x.abs() > size {
                size = p.x.abs();
            }
            if p.y.abs() > size {
                size = p.y.abs();
            }
        }
       for p in &polys[1] {
           if p.x.abs() > size {
               size = p.x.abs();
           }
           if p.y.abs() > size {
               size = p.y.abs();
           }
       } 
        size = 300f64/size; 
                //let mut tjek = polygon_generator::linecross_tjekker(p1, p2);
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                let center = c.transform.trans(300.0, 300.0)
                            .scale(1.0, -1.0);
                //let square = &[[1.0f64, 1.0f64],[1.0f64,30.0f64],[30.0f64,1.0f64]];
                //let trekant = &[[-1.0f64, -1.0f64],[-1.0f64,-30.0f64],[-30.0f64,-1.0f64]];
                    //let punkter = [square[2][0],square[2][1],trekant[2][0],trekant[2][1]];
                    let red = [1.0, 0.0, 0.0, 1.0];
                    let blue = [0.0, 0.0, 1.0, 1.0];
                    let green = [0.0, 1.0, 1.0, 1.0];
                    //polgon(red, square, center, g);
                    //polygon(red, trekant, center, g);
                    
                    for i in 1..n {
                        line(blue, 1.0, [polys[0][i-1].x*size as f64, polys[0][i-1].y*size as f64, polys[0][i].x * size as f64, polys[0][i].y * size as f64], center, g);
                    }
                    line(red, 1.0, [polys[0][n-1].x*size as f64, polys[0][n-1].y*size as f64, polys[0][n].x * size as f64, polys[0][n].y * size as f64], center, g);
                    
                    if n > 2 {
                        for i in 0..poly1.len()-1 {
                            line(green, 1.0, [poly1[i].x*size as f64, poly1[i].y*size as f64, poly1[i+1].x * size as f64, poly1[i+1].y * size as f64], center, g);
                        }
                    }
                   
                    /*for i in 0..polys[1].len() - 1 {
                        line(red, 1.0, [polys[1][i].x*size as f64, polys[1][i].y*size as f64, polys[1][i+1].x * size as f64, polys[1][i+1].y * size as f64], center, g);
                    }
                    
                    
                    for i in 0..poly2.len()-1 {
                        line(green, 1.0, [poly2[i].x*size as f64, poly2[i].y*size as f64, poly2[i+1].x * size as f64, poly2[i+1].y * size as f64], center, g);
                    }  ^&*/ /*k
                    for i in 0..tjek.len()-1 {
                        line(blue, 1.0, [tjek[i][0].x as f64, tjek[i][0].y as f64, tjek[i][1].x as f64, tjek[i][1].y as f64], center, g);
                        line(red, 1.0, [tjek[i][2].x as f64, tjek[i][2].y as f64, tjek[i][3].x as f64, tjek[i][3].y as f64], center, g);
                    }   */
            });
                  // Input loop
        if let Some(press_args) = e.press_args() {
            match press_args {
                Button::Keyboard(Key::Right) => n += 1,
                Button::Keyboard(Key::Left) => { if n > 0 {n -= 1;}}
                _ => (),
            }
            let v = polys[0][n];
            // her skal jeg implementere noget til at printe så jeg kan finde fejlen. I
            if !plane(v, poly1[0], poly1[1])  || !plane(poly1[poly1.len()-2], poly1[poly1.len()-1], v) {
                while !plane(poly1[poly1.len()-2], poly1[poly1.len()-1], v) {
                    println! ("pop_back");
                    poly1.pop_back();
                }
                poly1.push_back(v);
                while right_colinear_left(v, poly1[0], poly1[1]) != 1  {
                    println! ("pop_front");
                    poly1.pop_front();
                }
                poly1.push_front(v);
                println! (" front {} {}", poly1[1].x, poly1[1].y);
                println! (" back {} {}", poly1[poly1.len()-1].x, poly1[poly1.len()-1].y);
            }
    }
    
            /*
            let mut poly = polys[0].to_vec();
            let poly_n = poly.drain(..n).collect();
            println! ("polys 0 length: {}", polys[0].len());
            poly1 = melkmanns::algorithm_hull(poly_n);
            */
        }
    }
}

pub fn draw_grid_poly (polys: &Vec<Vec<Point>>) {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new(
        "Convexhull",
        [600, 600]
    )
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_lazy(true);
    let mut size = 0.0f64;
    if polys[0].len() > 2 && polys[1].len() > 2 {
        for p in &polys[0] {
            if p.x.abs() > size {
                size = p.x.abs();
            }
            if p.y.abs() > size {
                size = p.y.abs();
            }
        }
       for p in &polys[1] {
           if p.x.abs() > size {
               size = p.x.abs();
           }
           if p.y.abs() > size {
               size = p.y.abs();
           }
       } 


        let grid_size = (size/100.0).floor() as usize + 1;
        println! ("grid_size {}", grid_size);
        size = 300f64/(grid_size as f64 * 100.0);
                //let mut tjek = polygon_generator::linecross_tjekker(p1, p2);
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                let center = c.transform.trans(300.0, 300.0)
                            .scale(1.0, -1.0);
                //let square = &[[1.0f64, 1.0f64],[1.0f64,30.0f64],[30.0f64,1.0f64]];
                //let trekant = &[[-1.0f64, -1.0f64],[-1.0f64,-30.0f64],[-30.0f64,-1.0f64]];
                    //let punkter = [square[2][0],square[2][1],trekant[2][0],trekant[2][1]];
                    let red = [1.0, 0.0, 0.0, 1.0];
                    let blue = [0.0, 0.0, 1.0, 1.0];
                    let green = [0.0, 1.0, 1.0, 1.0];
                    let yellow = [0.0, 1.0, 0.0, 1.0];
                    //polgon(red, square, center, g);
                    //polygon(red, trekant, center, g);
                for i in 0..grid_size {
                    line(green, 1.0, [300.0 , i as f64 * 300.0/grid_size as f64 , -300.0 , i as f64 * 300.0/grid_size as f64], center, g);
                    line(green, 1.0, [300.0 , -(i as f64 * 300.0/grid_size as f64) , -300.0 , -(i as f64 * 300.0/grid_size as f64)], center, g);
                    line(green, 1.0, [i as f64 * 300.0/grid_size as f64 , 300.0 , i as f64 * 300.0/grid_size as f64, -300.0], center, g);
                    line(green, 1.0, [-(i as f64 * 300.0/grid_size as f64) , 300.0 , -(i as f64 * 300.0/grid_size as f64), -300.0], center, g);
                }
                    //line(yellow, 1.0, [i as f64 * size/300.0, 300.0, i as f64 * size/300.0, 300.0], center, g);
                //}

                for i in 0..polys[0].len()-1 {
                       line(blue, 1.0, [polys[0][i].x*size as f64, polys[0][i].y*size as f64, polys[0][i+1].x * size as f64, polys[0][i+1].y * size as f64], center, g);
                    }
                    
                   
                    for i in 0..polys[1].len() - 1 {
                        line(red, 1.0, [polys[1][i].x*size as f64, polys[1][i].y*size as f64, polys[1][i+1].x * size as f64, polys[1][i+1].y * size as f64], center, g);
                    }
            });
        }
    }
}

pub fn draw_aw (polys: &Vec<Vec<Point>>) {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new(
        "Convexhull",
        [600, 600]
    )
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_lazy(true);
    let mut size = 0.0f64;
    if polys[0].len() > 2 && polys[1].len() > 2 {
        for p in &polys[0] {
            if p.x.abs() > size {
                size = p.x.abs();
            }
            if p.y.abs() > size {
                size = p.y.abs();
            }
        }
       for p in &polys[1] {
           if p.x.abs() > size {
               size = p.x.abs();
           }
           if p.y.abs() > size {
               size = p.y.abs();
           }
       } 
        size = 300f64/size;
        let alpha = [-1, -1];
        let tangentt = algorithm(alpha, polys);
        let alpha = [1, 1];
        let tangentj = algorithm(alpha, polys);
        let alpha = [1, -1];
        let tangentm = algorithm(alpha, polys);
        let alpha = [-1, 1];
        let tangentn = algorithm(alpha, polys);
                //let mut tjek = polygon_generator::linecross_tjekker(p1, p2);
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                let center = c.transform.trans(300.0, 300.0)
                            .scale(1.0, -1.0);
                //let square = &[[1.0f64, 1.0f64],[1.0f64,30.0f64],[30.0f64,1.0f64]];
                //let trekant = &[[-1.0f64, -1.0f64],[-1.0f64,-30.0f64],[-30.0f64,-1.0f64]];
                    //let punkter = [square[2][0],square[2][1],trekant[2][0],trekant[2][1]];
                    let red = [1.0, 0.0, 0.0, 1.0];
                    let blue = [0.0, 0.0, 1.0, 1.0];
                    let green = [0.0, 1.0, 1.0, 1.0];
                    //polgon(red, square, center, g);
                    //polygon(red, trekant, center, g);
                    
                    line(green, 1.0, [0.0 , -300.0*size, 0.0 , 300.0*size], center, g);
                    line(green, 1.0, [-300.0*size, 0.0, -300.0*size, 0.0], center, g);

                    for i in 0..polys[0].len()-1 {
                       line(blue, 1.0, [polys[0][i].x*size as f64, polys[0][i].y*size as f64, polys[0][i+1].x * size as f64, polys[0][i+1].y * size as f64], center, g);
                    }
                    
                   
                    for i in 0..polys[1].len() - 1 {
                        line(red, 1.0, [polys[1][i].x*size as f64, polys[1][i].y*size as f64, polys[1][i+1].x * size as f64, polys[1][i+1].y * size as f64], center, g);
                    }
                     
                   
                    if let Some((t0, t1)) = tangentt {
                        line(green, 1.0, [polys[0][t0].x*size as f64, polys[0][t0].y*size as f64, polys[1][t1].x * size as f64, polys[1][t1].y * size as f64], center, g);
                    }
                    if let Some((n0, n1)) = tangentn {
                        line(green, 1.0, [polys[0][n0].x*size as f64, polys[0][n0].y*size as f64, polys[1][n1].x * size as f64, polys[1][n1].y * size as f64], center, g);
                        }
                    if let Some((j0, j1)) = tangentj {
                        line(green, 1.0, [polys[0][j0].x*size as f64, polys[0][j0].y*size as f64, polys[1][j1].x * size as f64, polys[1][j1].y * size as f64], center, g);
                    }
                    if let Some((m0, m1)) = tangentm {
                        line(green, 1.0, [polys[0][m0].x*size as f64, polys[0][m0].y*size as f64, polys[1][m1].x * size as f64, polys[1][m1].y * size as f64], center, g);
                    }
            });
/*
        if let Some(press_args) = e.press_args() {
                match press_args {
                    Button::Keyboard(Key::Right) => {
                        
                    if s[0] < 2*n[0] && s[1] < 2*n[1] && (v[0] < s[0] + n[0] || v[1] < s[1] + n[1]) {
                            // v_u = v_u + 1
                        v[u] = v[u] + 1;
                            // if p_u[v_u] not ∈ H_u(p_0[s_0],p_1[s_1]) and not b_u
                        if !(alpha[u] as f64 * super::util::det_star(polys[0][s[0] % n[0]], polys[1][s[1] % n[1]], polys[u][v[u] % n[u]]) <= 0.0) && !b[u] {   
                            println!("u:{} vu:{} su:{}", u, v[u], s[u]);
                            // p_(1−u)[s_(1−u)]∈∆(p_u[s_u],p_u[v_u−1],p_u[v_u])
                            if super::util::in_triangle(polys[1 - u][s[1-u] % n[1-u]], polys[u][s[u] % n[u]], polys[u][v[u]-1 % n[u]], polys[u][v[u] % n[u]]) {
                                println!("in triangle");
                                b[u] = true;
                            } else {
                                s[u] = v[u];
                                v[1-u] = s[1-u];
                                b[1-u] = false;
                            }
                        }
                    u = 1 - u;
                    }
                    }
                    _ => (),
                }
                }
*/
            }
    }
}

pub fn draw_aw_stepbystep (polys: &Vec<Vec<Point>>, alpha: &[i8;2]) {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new(
        "Convexhull",
        [600, 600]
    )
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_lazy(true);
    let mut size = 0.0f64;
    let a = alpha;
    let n = vec!(polys[0].len() as i64, polys[1].len() as i64);
    
    let (mut s , i) : ([i64;2], [i8;2]) = match (a[0], a[1]) {
        | (1, 1)    =>    ([n[0]-1 , 0],      [-1, 1]),
        | (-1, -1)  =>    ([0, n[1]-1],       [1, -1]),
        | (1, -1)   =>    ([0, 0],            [1, 1]),
        | (-1, 1)   =>    ([n[0]-1, n[1]-1],  [-1, -1]),
        | _         =>    ([0, n[1]-1],       [1, -1]),
}; 
    let mut v = s.to_vec();
    let mut b = vec!(false,false); 
    let mut u = 0;
    if polys[0].len() > 2 && polys[1].len() > 2 {
        for p in &polys[0] {
            if p.x.abs() > size {
                size = p.x.abs();
            }
            if p.y.abs() > size {
                size = p.y.abs();
            }
        }
       for p in &polys[1] {
           if p.x.abs() > size {
               size = p.x.abs();
           }
           if p.y.abs() > size {
               size = p.y.abs();
           }
       } 
        size = 300f64/size;
        
        let alpha = [-1, 1];
        let _tangentn = algorithm(alpha, polys);
                //let mut tjek = polygon_generator::linecross_tjekker(p1, p2);
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                let center = c.transform.trans(300.0, 300.0)
                            .scale(1.0, -1.0);
                //let square = &[[1.0f64, 1.0f64],[1.0f64,30.0f64],[30.0f64,1.0f64]];
                //let trekant = &[[-1.0f64, -1.0f64],[-1.0f64,-30.0f64],[-30.0f64,-1.0f64]];
                    //let punkter = [square[2][0],square[2][1],trekant[2][0],trekant[2][1]];
                    let red = [1.0, 0.0, 0.0, 1.0];
                    let blue = [0.0, 0.0, 1.0, 1.0];
                    let green = [0.0, 1.0, 1.0, 1.0];
                    let darkred = [1.0, 1.0, 1.0, 1.0];
                    //polgon(red, square, center, g);
                    //polygon(red, trekant, center, g);
                    //draw polygons
                    for j in 0..polys[0].len()-1 {
                       line(blue, 1.0, [polys[0][j].x*size as f64, polys[0][j].y*size as f64, polys[0][j+1].x * size as f64, polys[0][j+1].y * size as f64], center, g);
                    }
                    
                   
                    for j in 0..polys[1].len() - 1 {
                        line(red, 1.0, [polys[1][j].x*size as f64, polys[1][j].y*size as f64, polys[1][j+1].x * size as f64, polys[1][j+1].y * size as f64], center, g);
                    }
                    //draw dots
                    let dot1        =  polys[u][modular(v[u]+i[u] as i64, n[u])];
                    let cirle1      = ellipse::circle( (&dot1).x*size, (&dot1).y*size, 0.2f64 *size );
                    ellipse(green, cirle1, center, g);

                    let dot2 =  polys[1-u][modular(v[1-u]+i[1-u] as i64,n[1-u])];
                    let cirle2 = ellipse::circle( (&dot2).x*size, (&dot2).y*size, 0.2f64 *size );
                    ellipse([0.0, 0.5, 0.5, 0.5], cirle2, center, g);

                    //draw potential tangent
                    line(green, 1.0, [polys[0][modular(s[0],n[0])].x*size as f64, polys[0][modular(s[0],n[0])].y*size as f64, polys[1][modular(s[1],n[1])].x * size as f64, polys[1][modular(s[1],n[1])].y * size as f64], center, g);
                    
                    //draw triangle
                    if in_triangle(polys[1 - u][modular(s[1-u], n[1-u])], polys[u][modular(s[u], n[u])], polys[u][modular(v[u]-1, n[u])], polys[u][modular(v[u], n[u])]) {
                        line(darkred, 1.0, [polys[u][modular(s[u], n[u])].x*size as f64, polys[u][modular(s[u], n[u])].y*size as f64, polys[u][modular(v[u]-1, n[u])].x * size as f64, polys[u][modular(v[u]-1, n[u])].y * size as f64], center, g);
                        line(darkred, 1.0, [polys[u][modular(v[u], n[u])].x*size as f64, polys[u][modular(v[u], n[u])].y*size as f64, polys[u][modular(v[u]-1, n[u])].x * size as f64, polys[u][modular(v[u]-1, n[u])].y * size as f64], center, g);
                        line(darkred, 1.0, [polys[u][modular(s[u], n[u])].x*size as f64, polys[u][modular(s[u], n[u])].y*size as f64, polys[u][modular(v[u], n[u])].x * size as f64, polys[u][modular(v[u], n[u])].y * size as f64], center, g);
                    }
            });
        if let Some(press_args) = e.press_args() {
                match press_args {
                    Button::Keyboard(Key::Right) => {
                        if s[0].abs() < 2*n[0] &&
                        (s[1].abs() ) < 2*n[1] &&
                        (v[0].abs() < s[0].abs() + n[0] || v[1].abs() < s[1].abs()  + n[1]) {
                            // v_u = v_u + 1
                            v[u] = v[u] + i[u] as i64;
                            
                            // if p_u[v_u] not ∈ H_u(p_0[s_0],p_1[s_1]) and not b_u
                            if !( a[u] as f64 * det_star(polys[0][modular(s[0],n[0])], polys[1][modular(s[1], n[1])], polys[u][modular(v[u], n[u])]) <= 0.0) && !b[u] {
                                // p_(1−u)[s_(1−u)]∈∆(p_u[s_u],p_u[v_u−1],p_u[v_u])
                                if in_triangle(polys[1 - u][modular(s[1-u], n[1-u])], polys[u][modular(s[u], n[u])], polys[u][modular(v[u]-1, n[u])], polys[u][modular(v[u], n[u])]) {
                                    b[u] = true;
                                } else {
                                    s[u] = v[u];
                                    v[1-u] = s[1-u];
                                    b[1-u] = false;
                                }
                        }
                        u = 1 - u;
                    }
                    }
                    _ => (),
                }
                }

            }
    }
}


pub fn draw_deque_static (polys: &Vec<Vec<Point>>, show_hull: bool) {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new(
        "Convexhull",
        [600, 600]
    )
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_lazy(true);
    let mut size = 0.0f64;
    let mut n = 0;
    if polys[0].len() > 2 && polys[1].len() > 2 {
        for p in &polys[0] {
            if p.x.abs() > size {
                size = p.x.abs();
            }
            if p.y.abs() > size {
                size = p.y.abs();
            }
        }
       for p in &polys[1] {
           if p.x.abs() > size {
               size = p.x.abs();
           }
           if p.y.abs() > size {
               size = p.y.abs();
           }
       } 
        size = 300f64/size; 
        let poly1 = melkmanns::algorithm_hull(&polys[0]);
        let poly2 = melkmanns::algorithm_hull(&polys[1]);
        let mut hull1 = Vec::from(poly1);
        let mut hull2 = Vec::from(poly2);
        hull1.reverse();
        hull2.reverse();
        println! ("hull1 len: {}, hull2 len: {}", hull1.len(), hull2.len());
        let red = [1.0, 0.0, 0.0, 1.0];
        let blue = [0.0, 0.0, 1.0, 1.0];
        let green = [0.0, 1.0, 1.0, 1.0];
        let something = [1.0, 1.0, 0.0, 1.0];
        let yellow = [0.5, 0.5, 0.0, 1.0];
        let notyellow = [0.5, 0.0, 0.5, 1.0]; 
        let (l1, u1) = generate_u_l(&hull1);
        let (l2, u2) = generate_u_l(&hull2);
        let mut prev_point = Point{x: 0.0, y: 0.0};
        let mut current_point = u2[u2.len()-1];
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                let center = c.transform.trans(300.0, 300.0)
                            .scale(1.0, -1.0);
                //let square = &[[1.0f64, 1.0f64],[1.0f64,30.0f64],[30.0f64,1.0f64]];
                //let trekant = &[[-1.0f64, -1.0f64],[-1.0f64,-30.0f64],[-30.0f64,-1.0f64]];
                //let punkter = [square[2][0],square[2][1],trekant[2][0],trekant[2][1]];
                   
                    //polygon(red, trekant, center, g);
                     
                    for i in 0..polys[0].len()-1 {
                        line(blue, 1.0, [polys[0][i].x*size as f64, polys[0][i].y*size as f64, polys[0][i+1].x * size as f64, polys[0][i+1].y * size as f64], center, g);
                    }
                    for i in 0..polys[1].len() - 1 {
                        line(red, 1.0, [polys[1][i].x*size as f64, polys[1][i].y*size as f64, polys[1][i+1].x * size as f64, polys[1][i+1].y * size as f64], center, g);
                    }
                    if show_hull {
                        for i in 0..hull1.len()-1 {
                        line(green, 1.0, [hull1[i].x*size as f64, hull1[i].y*size as f64, hull1[i+1].x * size as f64, hull1[i+1].y * size as f64], center, g);
                        }
                    
                        for i in 0..l1.len() - 1 {
                            line(yellow, 1.0, [l1[i].x*size as f64, l1[i].y*size as f64, l1[i+1].x * size as f64, l1[i+1].y * size as f64], center, g);
                        }
                        for i in 0..u1.len() - 1 {
                            line(notyellow, 1.0, [u1[i].x*size as f64, u1[i].y*size as f64, u1[i+1].x * size as f64, u1[i+1].y * size as f64], center, g);
                        }
                        
                        for i in 0..hull2.len()-1 {
                            line(green, 1.0, [hull2[i].x*size as f64, hull2[i].y*size as f64, hull2[i+1].x * size as f64, hull2[i+1].y * size as f64], center, g);
                        }
                        for i in 0..l2.len() - 1 {
                            line(yellow, 1.0, [l2[i].x*size as f64, l2[i].y*size as f64, l2[i+1].x * size as f64, l2[i+1].y * size as f64], center, g);
                        }
                        for i in 0..u2.len() - 1 {
                            line(notyellow, 1.0, [u2[i].x*size as f64, u2[i].y*size as f64, u2[i+1].x * size as f64, u2[i+1].y * size as f64], center, g);
                        }
                        if n > 0 { 
                            line(something, 1.0, [prev_point.x*size as f64, prev_point.y*size as f64, current_point.x * size as f64, current_point.y * size as f64], center, g);
                        }
                    }
            });
            if let Some(press_args) = e.press_args() {
                match press_args {
                    Button::Keyboard(Key::Right) => {
                        n += 1;
                        prev_point = current_point;
                        if n % 2 == 1 {
                            let value = tangent_from_point(current_point, &u1, &l1, true).unwrap_or((true, usize::MAX));
                            if value.1 == usize::MAX {
                                println! ("tangent not found on poly2");
                            } else {
                                current_point = if value.0 { u1[value.1]} else { l1[value.1]};
                            }
                        } else {
                            let value = tangent_from_point(current_point, &u2, &l2, false).unwrap_or((true, usize::MAX));
                            if value.1 == usize::MAX {
                                println! ("tangent not found on poly1");
                            } else {
                                current_point = if value.0 { u2[value.1]} else { l2[value.1]};
                            }
                        }
                        }
                    _ => (),
                }
                
            }
        }
    }
}



pub fn draw_catapiller (cata: &Vec<Vec<Vec<Point>>> ) {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new(
        "Convexhull",
        [600, 600]
    )
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_lazy(true);
    /*let mut size = 0.0;
    if cata[0].len() > 2 && cata[1].len() > 2{
        for e in &cata[0] {
            for p in e {
                if p.x.abs() > size {
                    size = p.x.abs();
                }
                if p.y.abs() > size {
                    size = p.y.abs();
                }
            }
        }
       for e in &cata[1] {
            for p in e {
                if p.x.abs() > size {
                    size = p.x.abs();
                }
                if p.y.abs() > size {
                    size = p.y.abs();
                }
            }
       }
    size = 300f64/size; */
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            let center = c.transform.trans(300.0, 300.0)
                        .scale(1.0, -1.0);
            //let square = &[[1.0f64, 1.0f64],[1.0f64,30.0f64],[30.0f64,1.0f64]];
            //let trekant = &[[-1.0f64, -1.0f64],[-1.0f64,-30.0f64],[-30.0f64,-1.0f64]];
                //let punkter = [square[2][0],square[2][1],trekant[2][0],trekant[2][1]];
                let red = [1.0, 0.0, 0.0, 1.0];
                let blue = [0.0, 0.0, 1.0, 1.0];
                //let green = [0.0, 1.0, 1.0, 1.0];
                //polgon(red, square, center, g);
                //polygon(red, trekant, center, g);
                
                for edges in &cata[0] {
                    line(blue, 1.0, [edges[0].x as f64, edges[0].y as f64, edges[1].x  as f64, edges[1].y  as f64], center, g);
                }
                for edges in &cata[1] {
                    line(red, 1.0, [edges[0].x as f64, edges[0].y as f64, edges[1].x  as f64, edges[1].y  as f64], center, g);
                }
        });
    }
    //}
}
