use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent2019::intcode;

fn bench_intcode(c: &mut Criterion) {
    c.bench_function("series", |b| {
        let program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let s = intcode::Series::new(&program, 5);
        b.iter(|| {
            black_box(s.execute(vec![4, 3, 2, 1, 0]));
        })
    });
}

criterion_group!(benches, bench_intcode);
criterion_main!(benches);
