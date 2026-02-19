use std::io; // to use input output we take it from the standard library
// prelude aims to strike a balance by importing all the essentials that every 
// rust program might need without having to explicitly import it

fn main() {
    println!("Guess the number game!");
    // the exclamation mark calls a macro in Rust
    println!("Please input a number: ");

    // declaring a variable, a mutable variable, whose value can change
    let mut guess = String::new(); // returns new instance of a string, apply the new function on the string via the :: syntax

    // calling the standard input from the io library
    // could be written as std::io::stdin
    io::stdin()
        .read_line(&mut guess) // method to handle input from the user - take what the sax users says and append it to the string 
        .expect("Failed to real line");
    // curly braces - placeholder for dynamic data
    println!("ou guessed: {guess}")

    // immutability from what I understand is pointing a variable to a value that cannot be changed so when you do use a method to modify the contents of the variable, instead a new object is created and the variable points to the new object and the old object is destroyed
}