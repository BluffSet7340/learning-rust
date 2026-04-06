fn main() {
    println!("Hello, world!");
    another_function(5, 'a');

    // let mut x = 5;
    // let y =7; // does not return a value

    // x = y;

    // println!("value of x: {x}")

    // statement do not returns a value that's what expressions are for 
    let y = {
        let x = 4; // this is a statement
        x+1 // this works since this is an expression, the value of x is not being changed, just the resultant value is assigned to y
    };

    // let eb = 90;
    // let mut value = 8;
    // value = eb+1738;

    // println!("Result of {value}");


    // println!("Result of {y} after expression")
    let value = return_five();

    println!("here is this returned number {value}");

    let value_2 = plus_one(7);
    println!("Here is the value after adding one to it: {value_2}"); 

}

fn plus_one(num: i32) -> i32 {
    num+1
}

fn return_five() -> i32 {
    5
}

// function signature must have explicitly annotated types
fn another_function(num: i32, eb: char) {
    println!("Two values: {num}, {eb}");
}