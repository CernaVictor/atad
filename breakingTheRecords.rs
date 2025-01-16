use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'breakingRecords' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY scores as parameter.
 */

fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    let mut max_score = scores[0]; // Initialize with the first score
    let mut min_score = scores[0]; // Initialize with the first score
    let mut max_breaks = 0;
    let mut min_breaks = 0;

    // Iterate over the scores starting from the second game
    for &score in &scores[1..] {
        if score > max_score {
            max_score = score;
            max_breaks += 1; // Increment max_breaks if a new record is set
        } else if score < min_score {
            min_score = score;
            min_breaks += 1; // Increment min_breaks if a new record is set
        }
    }

    vec![max_breaks, min_breaks]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read the number of games
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the scores for the games
    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Get the result from breakingRecords function
    let result = breakingRecords(&scores);

    // Write the result to the output file
    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
