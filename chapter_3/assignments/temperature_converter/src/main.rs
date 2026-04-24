use std::io;

fn main() {
    // The program must take a number input from the user
    // the input must be converted from farenheit to celsius and displayed to the user

    loop {
        println!("Enter a number: ");

        let mut input = String::new(); // returns new instance of a string, apply the new function on the string via the :: syntax

        io::stdin()
            .read_line(&mut input) // mutable reference that I can modify
            .expect("Failed to read line");

        let number: f32 = match input.trim().parse() {
            // we use the match call and try to match either of the result enum => Ok or Err
            Ok(num) => num, // assign number and move on
            Err(_) => {
                // catch all for errors
                println!("Input a number! xD");
                continue; // move on to next iter
            }
        };

        let number = (number - 32.0) * 5.0 / 9.0;

        println!("here is the temperature from F to C = {number}");
        println!();
        println!("Press ctr+c to break out of the loop");
        println!();
    }
}
