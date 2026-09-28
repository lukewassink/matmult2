use super::*;

fn approx_compare(a: &FlatMatrix, b: &FlatMatrix, tolerance: f64) -> bool {
    if a.rows != b.rows || a.cols != b.cols {
        return false;
    }
    if a.vals.len() != b.vals.len() {
        return false;
    }

    for i in 0..(a.vals.len()) {
        if (a.vals[i] - b.vals[i]).abs() > tolerance {
            return false;
        }
    }

    true
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

                    println!("\n\n tile size: {}", tile_size);
                    println!("a: {}", a.print());
                    println!("b: {}", b.print());
                    println!("naive prod: {}", naive_mult(&a, &b).print());
                    println!("tiled prod: {}", tiled_mult(&a, &b, tile_size).print());

                    assert!(approx_compare(
                        &naive_mult(&a, &b),
                        &tiled_mult(&a, &b, tile_size),
                        0.00001
                    ));
                }
            }
        }
    }
}
