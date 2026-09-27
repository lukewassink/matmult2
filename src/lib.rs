use rand::{RngExt, SeedableRng, rngs::SmallRng};
use std::cmp::min;
use std::mem::swap;

pub struct FlatMatrix {
    rows: usize,
    cols: usize,
    vals: Vec<f64>,
}

impl FlatMatrix {
    pub fn new(rows: usize, cols: usize) -> FlatMatrix {
        let vals: Vec<f64> = vec![0.0; rows * cols];
        FlatMatrix { rows, cols, vals }
    }

    fn at(&self, row: usize, col: usize) -> f64 {
        self.vals[row * self.cols + col]
    }

    fn set(&mut self, row: usize, col: usize, val: f64) {
        self.vals[row * self.cols + col] = val;
    }

    fn transpose(&mut self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.vals.swap(i * self.cols + j, j * self.rows + i);
            }
        }
        swap(&mut self.rows, &mut self.cols);
    }
}

pub fn random_fill(m: &mut FlatMatrix, seed: u64) {
    let mut rng = SmallRng::seed_from_u64(seed);

    for val in m.vals.iter_mut() {
        *val = rng.random();
    }
}

fn validate_dimensions(m1: &FlatMatrix, m2: &FlatMatrix) {
    if m1.cols != m2.rows {
        panic!(
            "Cannot calculate product of matrices with missmatched dimensions: {} != {}",
            m1.rows, m2.cols
        );
    }
}

pub fn naive_mult(m1: &FlatMatrix, m2: &FlatMatrix) -> FlatMatrix {
    validate_dimensions(m1, m2);

    let mut out = FlatMatrix::new(m1.rows, m2.cols);

    for i in 0..m1.rows {
        for j in 0..m2.cols {
            for k in 0..m1.rows {
                out.set(i, j, out.at(i, j) + m1.at(i, k) * m2.at(k, j));
            }
        }
    }

    out
}

pub fn naive_local_mult(m1: &FlatMatrix, m2: &FlatMatrix) -> FlatMatrix {
    validate_dimensions(m1, m2);

    let mut out = FlatMatrix::new(m1.rows, m2.cols);

    for i in 0..m1.rows {
        for j in 0..m2.cols {
            let mut acc = 0.0;
            for k in 0..m1.rows {
                acc += m1.at(i, k) * m2.at(k, j);
            }
            out.set(i, j, acc);
        }
    }

    out
}

pub fn tiled_mult(m1: &FlatMatrix, m2: &FlatMatrix, tile_size: usize) -> FlatMatrix {
    validate_dimensions(m1, m2);

    let mut prod = FlatMatrix::new(m1.rows, m2.cols);

    for i in (0..m1.rows).step_by(tile_size) {
        for j in (0..m2.cols).step_by(tile_size) {
            for k in (0..m1.cols).step_by(tile_size) {
                for x in i..min(i + tile_size, prod.rows) {
                    for y in j..min(j + tile_size, prod.cols) {
                        let mut acc = 0.0;
                        for z in k..min(k + tile_size, m1.cols) {
                            acc += m1.at(x, z) * m2.at(z, y);
                        }
                        prod.set(i, j, acc + prod.at(i, j));
                    }
                }
            }
        }
    }

    prod
}
