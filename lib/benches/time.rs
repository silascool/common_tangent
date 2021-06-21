
use criterion::{ criterion_group, criterion_main, Criterion, BenchmarkId};
use libary::melkmanns;
use libary::aw;
use libary::polygon_generator::gen_polygon;
use libary::ghs;
use libary::util::Point;

fn melk_to_ghs (polys: &Vec<Vec<Point>>) -> Option<(usize,usize, bool)>{
    let v = vec!(melkmanns::algorithm_hull(&polys[0]), melkmanns::algorithm_hull(&polys[1]));
    ghs::ghs(&v)
}
/*
pub fn benchmark_melk(c: &mut Criterion) {
  

    let polys = gen_polygon(100);
    c.bench_function("Melkmann first poly", |b| b.iter(|| melk_to_ghs(&polys)));
}

pub fn benchmark_aw(c: &mut Criterion) {
    let polys = gen_polygon(100);
    c.bench_function("AW", |b| b.iter(|| aw::algorithm(vec!(-1i8, -1i8), &polys)));
    
}
*/
fn bench_aw_melk(c: &mut Criterion) {
    let mut group = c.benchmark_group("Melk vs AW");
    for i in  [10, 50, 100, 200, 500].iter() {
        let polys = gen_polygon(*i);
        group.bench_with_input(BenchmarkId::new("AW", i), &polys,
            |b, polys| b.iter(|| aw::algorithm([-1i8, -1i8], &polys)));
        group.bench_with_input(BenchmarkId::new("Melk", i), &polys,
            |b, polys| b.iter(|| melk_to_ghs(&polys)));
    }
    group.finish();
}

criterion_group!(benches, bench_aw_melk);
criterion_main!(benches);
