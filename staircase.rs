use std::io::{self, BufRead};

/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn staircase(n: i32) {
    for i in 1..=n {
        // Print the spaces
        let spaces = " ".repeat((n - i) as usize);
        // Print the '#' symbols
        let hashes = "#".repeat(i as usize);
        // Print the current row
        println!("{}{}", spaces, hashes);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Read the input size n
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Call the staircase function to print the staircase
    staircase(n);
}
