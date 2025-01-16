use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;
    
    let n = arr.len(); // Number of rows (and columns, since it's square)

    for i in 0..n {
        primary_diagonal_sum += arr[i][i]; // Primary diagonal element
        secondary_diagonal_sum += arr[i][n - 1 - i]; // Secondary diagonal element
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs() // Return the absolute difference
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n);

    // Read the matrix
    for i in 0..n {
        arr.push(
            stdin_iterator.next().unwrap().unwrap()
                .trim_end()
                .split(' ')
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        );
    }

    // Calculate the diagonal difference
    let result = diagonalDifference(&arr);

    // Write the result to the output file
    writeln!(&mut fptr, "{}", result).ok();
}
