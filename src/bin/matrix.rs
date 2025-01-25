use ndarray::Array2;
use rand::Rng;
use std::time::Instant;

fn main() {

    let cols = 2048;
    let rows = 2048;
    let mut rng = rand::thread_rng();
    let matrix_1 = Array2::from_shape_fn((cols,rows),  |_| rng.gen_range(-1000..=1000));
    let matrix_2 = Array2::from_shape_fn((cols,rows),  |_| rng.gen_range(-1000..=1000));
    let start = Instant::now();
    let _result = matrix_1.dot(&matrix_2);
    let duration = start.elapsed();
    println!("Multiplying matrices took: {:?}", duration);
}