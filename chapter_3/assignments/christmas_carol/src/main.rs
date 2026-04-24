fn main() {
    let days = ["second", "third", "fourth", "fifth", "sixth"];
    let items = [
        "Six geese a-laying,",
        "Five golden rings,",
        "Four calling birds,",
        "Three French hens,",
        "Two turtle doves",
    ];

    println!("On the first day of Christmas");
    println!("My true love gave to me");
    println!("A patridge in a pear tree.");
    println!();

    for day in days {
        println!("On the {day} day of Christmas");
        println!("My true love gave to me");
        if day == "second" {
            println!("{}", items[4]);
        }
        if day == "third" {
            println!("{}", items[3]);
            println!("{}", items[4]);
        }
        if day == "fourth" {
            println!("{}", items[2]);
            println!("{}", items[3]);
            println!("{}", items[4]);
        }
        if day == "fifth" {
            println!("{}", items[1]);
            println!("{}", items[2]);
            println!("{}", items[3]);
            println!("{}", items[4]);
        }
        if day == "sixth" {
            println!("{}", items[0]);
            println!("{}", items[1]);

            println!("{}", items[2]);

            println!("{}", items[3]);

            println!("{}", items[4]);
        }
        println!("And A patridge in a pear tree.");
        println!();
    }
}
