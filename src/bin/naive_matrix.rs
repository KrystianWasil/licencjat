use std::time::Instant;
use rand::Rng;

fn create_matrix(rows: i128, cols: i128) -> Vec<Vec<i128>> {
    let mut matrix = vec![vec![0; cols as usize]; rows as usize];
    let mut rng = rand::thread_rng();
    for i in 0..rows {
        for j in 0..cols {
            matrix[i as usize][j as usize] = rng.gen_range(-1000..1000);
        }
    }
    matrix
}

fn multiply_matrices_naive(rows: i128, cols: i128, matrix_1: &Vec<Vec<i128>>, matrix_2: &Vec<Vec<i128>>) -> Vec<Vec<i128>> {
    let mut result = vec![vec![0; cols as usize]; rows as usize];
    for i in 0..rows {
        for j in 0..cols {
            for k in 0..rows {
                result[i as usize][j as usize] += matrix_1[i as usize][k as usize] * matrix_2[k as usize][j as usize];
            }
        }
    }
    result
}
  
fn main() {
    let cols = 1024;
    let rows = 1024;
    let matrix_1 = create_matrix(rows, cols);
    let matrix_2 = create_matrix(rows, cols);
    
    //liczenie czasu mnożenia macierzy
    let start = Instant::now();
    let _result = multiply_matrices_naive(1024,1024,&matrix_1, &matrix_2); 
    let duration = start.elapsed();
    println!("Multiplying matrices took: {:?}", duration);

}
