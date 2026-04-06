use std::io;

fn main() {
    // let mut x = 1;
    // // type does not have to explicit, unless it is const variable

    // {
    //     let mut x = x;

    //     x += 2;
    // }
    // // when it goes out of scope, the topmost layer of the shadow is removed
    // // this means take whatever you're given and attempt to convert it to integer
    // let guess: u32 = "22".parse().expect("Not a number!");

    // println!("{x}");

    // const variables need to be uppercase
    // const X : i32 = 1738;

    // println!("{X}");

    let mut value = 1738;

    value = 67; // or you can do let mut value again but that defeats the purpose

    println!("{value}\n");

    // using let keyword allows you to redefine variable type

    let spaces = "      "; // string type
    let spaces = spaces.len(); // boom integer type

    println!("{spaces}");

    // //////////////////////// mut keyword does not let you change data type - can mutate value but not its type!

    let mut ebola = "virus";

    ebola = "anaconda";

    println!("{ebola}");

    // ////////////////////// numeric operations

    // two's complement wrapping is done there is an integer overflow

    let difference = 95.5 - 4.3;
    println!("\n{difference}");

    let quotient = 56.7 / 32.2;
    println!("{quotient}"); // since the numbers are floating point, answer is also in floating point

    let truncate = -5 / 3;
    println!("{truncate}"); // ans is -1 since you are dividing 2 integers so you get non decimal answer

    // remainder
    let remainder = 43 % 5;
    println!("{remainder}");

    let t = true; // implicit type annotion, the rust compiler figures it out for you
    let f: bool = false; // explicit type annotion

    println!("{f}");

    // char types - 4 bytes so 32 bits

    let ch = 'a'; // string uses double quotes, char uses single quotes
    let char: char = 'b'; // explicit type annotation again
    let heart_eyed_cat = '😻'; // so you can do emojis too
    let space = ' '; // empty char, can only be 1 space
    println!("{space}");

    // //////////////////////////////// tuples - compound types

    // tup binds to the entire compound type
    let tup: (i32, u8, f64) = (-20, 200, 0.678); // explicit annotation
                                                 // println!("{tup}");

    // get individual value out of tuple via pattern destructuring

    let tup = (500, 37, 0.98);
    let (x, y, z) = tup;

    {
        let x = 25; // so the shadow stuff still applies
    }

    println!("The first value is : {x}");

    let anotherValue = tup.1; // access directly from a tuple

    println!("The second value  to be accessed via a dot is {anotherValue}");

    let mut z: (i32, i32) = (1, 2);
    z.0 = 0;
    z.1 += 5;

    // final value of tuple is (0, 7)

    // ///////////////////// array type - compound type

    // fixed length and all values must be of the same type

    // let list = [1, 4, 6, 76];

    // println!("{x}")

    // println!("{list}");

    // let eb: [i32; 4] = [5, 6, 7, 255]; // first defines type and the second value defines number of elements, explicit

    // let test = [3; 5]; // first is the value to be used so 3 and the second value is the number of time it is repeated so [3, 3, 3, 3, 3]

    // let firstElt = test[0];

    // println!("first element is {firstElt}");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

        // interesting, slice indices of arrays have to be of type usize cannot be changed
    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    let index = 2;

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");



    // /////////////////////////////// Functions in Rust

    another_function()
}

fn another_function() {
    println!("Ebola virus")
}
