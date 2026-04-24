use std::io;

fn main() {
    let mut num_0 = 0;
    let mut num_1 = 1;
    let mut num_final = 0;
    let mut num_loops = 0;

    println!("Enter a number: ");

    let mut input = String::new(); // returns new instance of a string, apply the new function on the string via the :: syntax

    io::stdin()
        .read_line(&mut input) // mutable reference that I can modify
        .expect("Failed to read line");

    let mut number: u32 = match input.trim().parse() {
        // we use the match call and try to match either of the result enum => Ok or Err
        Ok(num) => num, // assign number and move on
        Err(_) => 0,
    };

    num_loops = number;

    while num_loops > 0 {
        if (number == 0) {
            break;
        }
        if (number == 1) {
            num_0 = num_0 + num_1;
            break;
        }
        num_final = num_0 + num_1;
        num_0 = num_1;
        num_1 = num_final;
        num_loops -= 1;
    }

    println!("The {number}th sequence of the Fibonacci sequence is {num_0}");
}
