use super::*;
use approx::assert_abs_diff_eq;

fn assert_approx_eq(a: &FlatMatrix, b: &FlatMatrix, tolerance: f64) {
    assert_eq!(a.rows, b.rows);
    assert_eq!(a.cols, b.cols);
    assert_eq!(a.vals.len(), b.vals.len());

    for i in 0..(a.vals.len()) {
        assert_abs_diff_eq!(a.vals[i], b.vals[i], epsilon = tolerance);
    }
}

#[test]
fn naive_mult_test() {
    let a = FlatMatrix {
        rows: 3,
        cols: 2,
        vals: vec![1., 2., 3., 4., 5., 6.],
    };
    let b = FlatMatrix {
        rows: 2,
        cols: 2,
        vals: vec![1., 2., 3., 4.],
    };
    let prod = FlatMatrix {
        rows: 3,
        cols: 2,
        vals: vec![7., 10., 15., 22., 23., 34.],
    };

    assert_eq!(naive_mult(&a, &b), prod);
}

#[test]
fn naive_local_test() {
    for rows in 1..5 {
        for cols in 1..5 {
            for inner in 1..5 {
                let mut a = FlatMatrix::new(rows, inner);
                let mut b = FlatMatrix::new(inner, cols);
                random_fill_random_seed(&mut a);
                random_fill_random_seed(&mut b);

                assert_eq!(naive_mult(&a, &b), naive_local_mult(&a, &b));
            }
        }
    }
}

#[test]
fn tiled_test() {
    for rows in 1..10 {
        for cols in 1..10 {
            for inner in 1..10 {
                for tile_size in 1..5 {
                    let mut a = FlatMatrix::new(rows, inner);
                    let mut b = FlatMatrix::new(inner, cols);
                    random_fill_random_seed(&mut a);
                    random_fill_random_seed(&mut b);

                    assert_approx_eq(&naive_mult(&a, &b), &tiled_mult(&a, &b, tile_size), 0.00001);
                }
            }
        }
    }
}

#[test]
fn par_tiled_test() {
    for rows in 1..10 {
        for cols in 1..10 {
            for inner in 1..10 {
                for tile_size in 1..5 {
                    for threads in 1..3 {
                        let mut a = FlatMatrix::new(rows, inner);
                        let mut b = FlatMatrix::new(inner, cols);
                        random_fill_random_seed(&mut a);
                        random_fill_random_seed(&mut b);

                        assert_approx_eq(
                            &naive_mult(&a, &b),
                            &&parallel_tiled_mult(&a, &b, tile_size, threads),
                            0.00001,
                        );
                    }
                }
            }
        }
    }
}
