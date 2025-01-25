use ndarray::{Array1,Array2,s};

static MAX_ITERATION: i32 = 1000;
static EPSILON: f64 = 1e-10;

fn main() {
    // Definicja macierzy A i wektora b
    let a: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> = Array2::from_shape_vec((4, 4), vec![
        10., -1., 2., 0.,
        -1., 11., -1., 3.,
        2., -1., 10., -1.,
        0.0, 3., -1., 8.,
    ]).unwrap();

    let b = Array1::from_vec(vec![6., 25., -11., 15.]);

    // Inicjalizacja wektora x
    let mut x = Array1::from_elem(4, 0.0);

    // Iteracje metody Jacobiego
    for iteration in 0..MAX_ITERATION {
        let mut x_new = x.clone();

        for i in 0..a.nrows() {
            // let mut s1 = 0.0;
            // let mut s2 = 0.0;

            // for j in 0..i {
            //      s1 += A[[i, j]] * x[j]; 
            // } 

            // Obliczanie sumy s1 (przed i)
            let s1 = a.slice(s![i, 0..i])
            .iter()
            .zip(x.slice(s![0..i]).iter())
            .map(|(&a, &b)| a * b)
            .sum::<f64>();

            // Obliczanie sumy s2 (po i)
            // for j in (i + 1)..A.ncols() {
            //     s2 += A[[i, j]] * x[j];
            // }
            let s2 = a.slice(s![i, (i + 1)..])
            .iter()
            .zip(x.slice(s![(i + 1)..]).iter())
            .map(|(&a, &b)| a * b)
            .sum::<f64>();

            // Aktualizacja wartości x_new[i]
            x_new[i] = (b[i] - s1 - s2) / a[[i, i]];
        }

        // Sprawdzanie warunku zbieżności
        if (&x_new - &x).iter().cloned().fold(f64::MIN, f64::max).abs() < EPSILON {
            println!("Rozwiązanie znalezione po {} iteracjach:", iteration + 1);
            
            // Konwersja wyników na liczby całkowite
            let x_int = x_new.mapv(|val| val.round() as i32);
            println!("{:?}", x_int);
            return;
        }

        // Aktualizacja wektora x
        x = x_new;
    }

    println!("Nie znaleziono rozwiązania po {} iteracjach.", MAX_ITERATION);
}