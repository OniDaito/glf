//!    ___  __    ____ 
//!   / __)(  )  (  __)
//!  ( (_ \/ (_/\ ) _) 
//!   \___/\____/(__) 
//!   
//! # Benchmark
//! Using the criterion crate (as rust bench is unstable) to benchmark our main functions
//! 
//! https://github.com/bheisler/criterion.rs

use std::{path::{PathBuf, Path}, time::Instant};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glf::GLF;


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("glf100", |b| {
        b.iter_custom( |iters| {

            let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            d.push("pytritech_testdata/test_tritech.glf");
            let glf = GLF::new(Path::new(&d)).unwrap();
            let start = Instant::now();
            
            for i in 0..iters {
                black_box(glf.extract_image(i as usize).unwrap());
            }
            start.elapsed()
        });
    }); 
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);