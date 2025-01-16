use std::io;

//compute sum of 2 integers
fn solve_me_first(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // variable declaration
    let mut _num_str_1 = String::new();
    let mut _num_str_2 = String::new();

    // read variables
    io::stdin().read_line(&mut _num_str_1).ok().expect("read error");
    io::stdin().read_line(&mut _num_str_2).ok().expect("read error");

    // parse integers
    let _num_1: i32 = _num_str_1.trim().parse().ok().expect("parse error");
    let _num_2: i32 = _num_str_2.trim().parse().ok().expect("parse error");

    // print the sum using the solve_me_first function
    println!("{}", solve_me_first(_num_1, _num_2));
}
