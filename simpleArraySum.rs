use std::io;

fn simple_array_sum(arr: Vec<i32>) -> i32 {
    arr.iter().sum() // Sum all the elements of the array
}

fn main() {
    let mut input = String::new();
    
    // Read the size of the array (we don't really need to use it)
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Read the array of integers
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input string into a vector of integers
    let arr: Vec<i32> = input
        .split_whitespace()  // Split the input by spaces
        .map(|s| s.parse().unwrap())  // Convert each split string to an i32
        .collect(); // Collect them into a vector

    // Call the function to compute the sum and print the result
    let result = simple_array_sum(arr);
    println!("{}", result);
}
