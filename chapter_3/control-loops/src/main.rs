fn main() {
    // println!("Hello, world!");
    let num = 4;

    // if num < 4 {
    //     println!("ebola"); // this is an arm
    // } else {
    //     println!("virus");
    // }

    // hmmm they expect bool but in other languages this is okay
    // if blocks only accept boolean as their condition
    if num != 1738 {
        println!("can you see me");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let conditional = false;
    // assignment via expression
    let ebola = if conditional {"virus"} else {"razor-thin"};
    println!("{ebola}");

    // let mut guess = "ebola";
    // guess = "test";
    // println!("{guess}");
    
}
