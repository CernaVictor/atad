use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
    let n = arr.len() as f64; // Total number of elements in the array

    // Count positive, negative, and zero values
    let (mut positive_count, mut negative_count, mut zero_count) = (0, 0, 0);

    for &num in arr {
        if num > 0 {
            positive_count += 1;
        } else if num < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    // Calculate the ratios
    let positive_ratio = positive_count as f64 / n;
    let negative_ratio = negative_count as f64 / n;
    let zero_ratio = zero_count as f64 / n;

    // Print the ratios with six decimal places
    println!("{:.6}", positive_ratio);
    println!("{:.6}", negative_ratio);
    println!("{:.6}", zero_ratio);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Read the size of the array (we don't actually need this value in Rust)
    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the array elements
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Call the plusMinus function
    plusMinus(&arr);
}
