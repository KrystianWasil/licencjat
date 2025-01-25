use ndarray::Array;
use rand::Rng;
use std::time::Instant;

//test
fn main() {
    let cols = 3;
    let rows = 3;
    let mut rng = rand::thread_rng();
    let matrix_1 = Array::from_shape_fn((cols,rows),  |_| rng.gen_range(0..=3));
    let matrix_2 = Array::from_shape_fn((cols,rows),  |_| rng.gen_range(0..=3));
    println!("{:?}", matrix_1);
    println!("{:?}", matrix_2);
    let start = Instant::now();
    let _result = matrix_1.dot(&matrix_2);
    let duration = start.elapsed();
    println!("Multiplying matrices took: {:?}", duration);
    print!("{:?}", _result);
}
