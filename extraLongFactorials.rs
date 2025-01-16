use std::io::{self, BufRead};

/// Function to calculate the factorial of `n` and print the result.
/// It uses a vector of digits to handle large numbers, simulating big integer multiplication.
fn extraLongFactorials(n: i32) {
    let mut result = vec![1]; // Initialize the result as 1 (since 0! = 1 and 1! = 1)

    for i in 2..=n {
        multiply_bigint(&mut result, i); // Multiply the current result by i
    }

    // Print the result (big integer in reverse order)
    for digit in result.iter().rev() {
        print!("{}", digit);
    }
    println!(); // For a new line after printing the result
}

/// Helper function to multiply a big integer (stored as a vector) by an integer `num`.
fn multiply_bigint(bigint: &mut Vec<u8>, num: i32) {
    let mut carry = 0; // The carry during multiplication

    for digit in bigint.iter_mut() {
        let product = (*digit as i32) * num + carry;
        *digit = (product % 10) as u8;
        carry = product / 10;
    }

    // If there is any remaining carry, add it to the result
    while carry > 0 {
        bigint.push((carry % 10) as u8);
        carry /= 10;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    extraLongFactorials(n);
}
