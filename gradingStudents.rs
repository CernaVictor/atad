use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'gradingStudents' function below.
 
 HackerLand University has the following grading policy:


Any grade less than 38 is a failing grade.

Sam is a professor at the university and likes to round each student's according to these rules:

If the difference between the grade and the next multiple of 5 is less than 3, round up to the next multiple of 5.

If the value of grade is less than 38, no rounding occurs as the result will still be a failing grade.

Examples

grade=84 -> round to (85 - 84 is less than 3)

grade=29 -> do not round (result is less than 38)

grade=57 ->do not round (60 - 57 is 3 or higher)
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY grades as parameter.
 */

fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| {
        if grade >= 38 {
            let next_multiple_of_5 = ((grade / 5) + 1) * 5;
            if next_multiple_of_5 - grade < 3 {
                next_multiple_of_5
            } else {
                grade
            }
        } else {
            grade
        }
    }).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let grades_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut grades: Vec<i32> = Vec::with_capacity(grades_count as usize);

    for _ in 0..grades_count {
        let grades_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        grades.push(grades_item);
    }

    let result = gradingStudents(&grades);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}