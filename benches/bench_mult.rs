use divan::{Bencher, black_box};
use matmult2::FlatMatrix;
use matmult2::naive_local_mult;
use matmult2::naive_mult;
use matmult2::random_fill;
use matmult2::tiled_mult;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

fn setup(size: usize) -> (FlatMatrix, FlatMatrix) {
    let mut a = FlatMatrix::new(size, size);
    let mut b = FlatMatrix::new(size, size);
    random_fill(&mut a, 1);
    random_fill(&mut b, 2);
    (a, b)
}

#[divan::bench]
#[ignore]
fn naive(bencher: Bencher) {
    let (a, b) = setup(1000);
    bencher.bench_local(|| {
        black_box(naive_mult(&a, &b));
    });
}

#[divan::bench]
#[ignore]
fn local_acc(bencher: Bencher) {
    let (a, b) = setup(1000);
    bencher.bench_local(|| {
        black_box(naive_local_mult(&a, &b));
    });
}

#[divan::bench(args = [8, 16, 32, 64, 128, 256, 512])]
fn tiled(bencher: Bencher, tile_size: usize) {
    let (a, b) = setup(1024);
    bencher.bench_local(|| {
        black_box(tiled_mult(&a, &b, tile_size));
    });
}
