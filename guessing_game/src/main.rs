use std::io;
// to use input output we take it from the standard library
// prelude aims to strike a balance by importing all the essentials that every
// rust program might need without having to explicitly import it
// use rand::Rng; // unused import so commenting it out
use std::cmp::Ordering;

fn main() {
    println!("Guess the number game!");
    // the exclamation mark calls a macro in Rust
    let sec_num = rand::random_range(1..=100); // generate number from 1 to 100

    // println!("The secret number is {sec_num}");

    println!("Please input a number: ");

    // calling the standard input from the io library
    // could be written as std::io::stdin
    // pressing enter adds a newline to the string
    loop {
        // declaring a variable, a mutable variable, whose value can change
        let mut guess = String::new(); // returns new instance of a string, apply the new function on the string via the :: syntax

        io::stdin()
            .read_line(&mut guess) // method to handle input from the user - take what the  users says and append it to the string
            .expect("Failed to read line"); // this runs when the enum result is err, else it returns the input 
        // read_line() returns an enum and encode info according to two state ok - and err.
        // ok is linked to ouput and err linked to error msg and why it failed
        // curly braces - placeholder for dynamic data

        // convert string to integer
        // the u32 is unsigned int, 32 bit, parse it as so
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            // we use the match call and try to match either of the result enum => Ok or Err
            Ok(num)=>num, // assign number and move on
            Err(_)=> continue, // catch any and all errors and move on to next iteration
        };
        // the .expect is there is handle any failures in parsing

        println!("You guessed this number: {guess}");

        // got a mismatch type error since we are taking in string as user input
        // but the secret number is a goddamn integer
        match guess.cmp(&sec_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal =>{
                println!("You guessed right!");
                break;
            } 
            Ordering::Greater => println!("Too large!"),
        }
    }

    // immutability from what I understand is pointing a variable to a value that cannot be changed so when you do use a method to modify the contents of the variable, instead a new object is created and the variable points to the new object and the old object is destroyed
}
