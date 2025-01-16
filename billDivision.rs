use std::io::{self, BufRead};

/*
 * Complete the 'bonAppetit' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY bill
 *  2. INTEGER k
 *  3. INTEGER b
 */

fn bonAppetit(bill: &[i32], k: i32, b: i32) {
    // Step 1: Calculate the total cost of the shared items (excluding the item at index `k`)
    let total_cost: i32 = bill.iter().enumerate()
        .filter(|&(i, _)| i != k as usize) // Filter out the item Anna didn't eat
        .map(|(_, &price)| price)         // Get the price of the remaining items
        .sum();                           // Sum up the remaining prices
    
    // Step 2: Calculate the fair share, which is half the total cost of the shared items
    let fair_share = total_cost / 2;
    
    // Step 3: Check if Brian overcharged Anna
    if b == fair_share {
        println!("Bon Appetit");
    } else {
        // Step 4: If overcharged, print the difference (what Brian owes Anna)
        println!("{}", b - fair_share);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Read first line with n (number of items) and k (the index of the item Anna doesn't eat)
    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();
    
    let _n = first_multiple_input[0].trim().parse::<i32>().unwrap(); // Total number of items (not needed explicitly)
    let k = first_multiple_input[1].trim().parse::<i32>().unwrap(); // Index of the item Anna didn't eat

    // Read the second line for the bill
    let bill: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Read the amount Brian charged Anna
    let b = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Call the function to determine if the charge is correct
    bonAppetit(&bill, k, b);
}
