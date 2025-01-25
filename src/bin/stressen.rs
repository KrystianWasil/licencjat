// use ndarray::{Array2, Axis, CowArray, Ix2, s};

// fn split<'a>(matrix: CowArray<'a, i64, Ix2>) -> (CowArray<'a, i64, Ix2>, CowArray<'a, i64, Ix2>, CowArray<'a, i64, Ix2>, CowArray<'a, i64, Ix2>) {
//     let (n, m) = matrix.dim(); // Get dimensions
//     let mid_n = n / 2; // Middle along axis 0 (rows)
//     let mid_m = m / 2; // Middle along axis 1 (columns)

//     // Work with the view variant of CowArray
//     let matrix_view = matrix.view();

//     // Split along axis 0 (top and bottom)
//     let (top, bottom) = matrix_view.split_at(Axis(0), mid_n);

//     // Split the top part along axis 1 (left and right)
//     let (top_left, top_right) = top.split_at(Axis(1), mid_m);

//     // Split the bottom part along axis 1 (left and right)
//     let (bottom_left, bottom_right) = bottom.split_at(Axis(1), mid_m);

//     // Convert the result back to CowArray, cloning if necessary
//     (
//         CowArray::from(top_left.to_owned()), 
//         CowArray::from(top_right.to_owned()), 
//         CowArray::from(bottom_left.to_owned()), 
//         CowArray::from(bottom_right.to_owned())
//     )
// }

// fn strassen<'a>(matrix_1: CowArray<'a, i64, Ix2>, matrix_2: CowArray<'a, i64, Ix2>) -> CowArray<'a, i64, Ix2> {
//     let (n, _) = matrix_1.dim();
//     if n <= 64 {
//         // Base case: perform dot product
//         let result = matrix_1.dot(&matrix_2);
//         return CowArray::from(result.to_owned());
//     } else {
//         // Split the matrices
//         let (a11, a12, a21, a22) = split(matrix_1);
//         let (b11, b12, b21, b22) = split(matrix_2);

//         // Recursively compute intermediate matrices
//         let m1 = strassen(a11 + a22, b11 + b22);
//         let m2 = strassen(a21 + a22, b11);
//         let m3 = strassen(a11, b12 - b22);
//         let m4 = strassen(a22, b21 - b11);
//         let m5 = strassen(a11 + a12, b22);
//         let m6 = strassen(a21 - a11, b11 + b12);
//         let m7 = strassen(a12 - a22, b21 + b22);

//         // Combine intermediate results to compute final blocks
//         let c11 = m1 + m4 - m5 + m7;
//         let c12 = m3 + m5;
//         let c21 = m2 + m4;
//         let c22 = m1 - m2 + m3 + m6;

//         // Reassemble the final result into a single matrix
//         let mut result = Array2::zeros((n, n));
//         result.slice_mut(s![..n / 2, ..n / 2]).assign(&c11);
//         result.slice_mut(s![..n / 2, n / 2..]).assign(&c12);
//         result.slice_mut(s![n / 2.., ..n / 2]).assign(&c21);
//         result.slice_mut(s![n / 2.., n / 2..]).assign(&c22);

//         return CowArray::from(result);
//     }
// }

// fn main() {
//     // Example 4x4 matrix
//     let matrix = Array2::from_shape_vec((4, 4), vec![
//         0, 1, 2, 3,
//         4, 5, 6, 7,
//         8, 9, 10, 11,
//         12, 13, 14, 15,
//     ])
//     .unwrap();

//     // Create CowArray from the matrix view (borrowed)
//     let cow_matrix = CowArray::from(matrix.view());

//     // Split the matrix into 4 parts
//     let (top_left, top_right, bottom_left, bottom_right) = split(cow_matrix);

//     // Print the results
//     println!("Top Left: \n{}", top_left);
//     println!("Top Right: \n{}", top_right);
//     println!("Bottom Left: \n{}", bottom_left);
//     println!("Bottom Right: \n{}", bottom_right);

//     // Perform matrix multiplication using Strassen
//     let result = strassen(top_left, top_right);
//     println!("Strassen Result: \n{}", result);
// }

use rand::Rng;
use std::time::Instant;
use ndarray::{s, Array2, ArrayView2, Axis};

fn add(a: ArrayView2<i64>,b:ArrayView2<i64>) -> Array2<i64> {
    let result = &a + &b;
    return result; //utworzyla nam sei nowa tablica
}

fn sub(a: ArrayView2<i64>, b: ArrayView2<i64>) -> Array2<i64> {
    let result = &a - &b;
    return result;
}

fn split<'a>(m: ArrayView2<i64>)->(ArrayView2<i64>,ArrayView2<i64>,ArrayView2<i64>,ArrayView2<i64>) {
    let (n,_) = m.dim();
    let mid_n = n/2;
    //axis split
    let (top,bottom) = m.split_at(Axis(0),mid_n);
    let (top_left,top_right) = top.split_at(Axis(1),mid_n);
    let (bottom_left,bottom_right) = bottom.split_at(Axis(1),mid_n);
    (top_left,top_right,bottom_left,bottom_right)
}

fn strassen(a: ArrayView2<i64>,b:ArrayView2<i64>) -> Array2<i64> {
    let (n,_) = a.dim();

    if n <=64 {return a.dot(&b)}
    else {
        let (a11,a12,a21,a22) = split(a);
        let (b11,b12,b21,b22) = split(b);

        let m1 = strassen(add(a11,a22).view(),add(b11,b22).view());
        let m2 = strassen(add(a21,a22).view(),b11);
        let m3 = strassen(a11,sub(b12,b22).view());
        let m4 = strassen(a22,sub(b21,b11).view());
        let m5 = strassen(add(a11,a12).view(),b22);
        let m6 = strassen(sub(a21,a11).view(),add(b11,b12).view());
        let m7 = strassen(sub(a12,a22).view(),add(b21,b22).view());

        let c11 = add(sub(add(m1.view(),m4.view()).view(),m5.view()).view(),m7.view());
        let c12 = add(m3.view(),m5.view());
        let c21 = add(m2.view(),m4.view());
        let c22 = add(sub(add(m1.view(),m3.view()).view(),m2.view()).view(),add(m6.view(),m7.view()).view());

        let mut result = Array2::zeros((n,n));
        result.slice_mut(s![..n/2,..n/2]).assign(&c11);
        result.slice_mut(s![..n/2,n/2..]).assign(&c12);
        result.slice_mut(s![n/2..,..n/2]).assign(&c21);
        result.slice_mut(s![n/2..,n/2..]).assign(&c22);

        return result;



        

    }

}

fn main(){
    let cols = 2048;
    let rows = 2048;
    let mut rng = rand::thread_rng();
    let matrix_1 = Array2::from_shape_fn((cols,rows),  |_| rng.gen_range(-1000..=1000));
    let matrix_2 = Array2::from_shape_fn((cols,rows),  |_| rng.gen_range(-1000..=1000));
    let start = Instant::now();
    let _result = strassen(matrix_1.view(),matrix_2.view());
    let duration = start.elapsed();
    println!("Multiplying matrices tookk: {:?}", duration);

}