use ndarray::{Array2, Axis};

fn main() {
    // tablice a i b z vectorow na array by meic odpowie4dni ksztalt
    let a: Array2<f64> = Array2::from_shape_vec((3, 3), vec![
        2.0, 1.0, -1.0,
        1.0, -1.0, 1.0,
        -3.0, 2.0, 0.0,
    ]).unwrap();
    
    let b: Array2<f64> = Array2::from_shape_vec((3, 1), vec![
        2.0, -5.0, 17.0,
    ]).unwrap();

    // zlaczenie tablic horyzontalnie
    let mut ab = ndarray::concatenate(Axis(1), &[a.view(), b.view()]).unwrap();

    let n = ab.shape()[0]; // wiersze

    // Gauss\-Jordan elimination
    for i in 0..n {
        // wybór wiersza z największym elementem w kolumnie i
        let mut pivot_row = i;
        for j in i + 1..n {
            if ab[[j, i]].abs() > ab[[pivot_row, i]].abs() {
                pivot_row = j;
            }
        }

        // zamiana wierszy
        if pivot_row != i {
            for k in 0..=n {
                ab.swap((i, k), (pivot_row, k));
            }
        }

        // normalizacja wiersza aby główny element był równy 1
        let pivot = ab[[i, i]];
        for k in i..=n {
            ab[[i, k]] /= pivot;
        }

        // eliminacja pozostałych wierszy
        for j in 0..n {
            if j != i {
                let factor = ab[[j, i]];
                for k in i..=n {
                    ab[[j, k]] -= factor * ab[[i, k]];
                }
            }
        }
    }

    // odczytanie rozwiązania jako kolumny
    let solution = ab.column(n).to_owned();

    // konwersja wyniku na liczby całkowite
    let solution_as_ints: Vec<i32> = solution.iter().map(|&x| x.round() as i32).collect();

    // wyświetlenie rozwiązania
    println!("Rozwiązanie (jako liczby całkowite): {:?}", solution_as_ints);
}
