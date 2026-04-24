fn main() {
    // loop {
    //     println!("Hello, world!");
    // }

    // let mut counter = 0;

    // let finalValue = loop {
    //     counter += 2;
    //     if counter>=1738 {
    //         break counter; // when you break out of the loop you have to return the number or value or whatever
    //     }
    // };

    // println!("The value is {finalValue}");

    // let mut counter = 0;

    // let finalValue = 'first_loop: loop {
    //     counter+=2;

    //     if counter == 4 {
    //         'second_loop: loop {
    //             counter+=3;

    //             if counter == 7 {
    //                 break 'second_loop;
    //             }
    //         }
    //     }

    //     if counter >= 1738 {
    //         break counter;
    //     }
    // };

    // println!("{finalValue}");

    //     let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    // 'counting_test: loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break 'counting_test; // adding name of label is redundant if you're exiting the inner loop regardless
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");

    // let mut num = 3;

    // while num != 0 {
    //     num -= 1;
    //     println!("hello {num} times")
    // }

    // println!("We have lift!");

    // while_loop();

    let array = [10, 20, 30, 40, 60, 70, 90];
    let mut index = 0;

    // while index < 7 {
    //     println!("{}", array[index]);
    //     index+=1;
    // }

    // for elt in array {
    //     println!("{elt}");
    // }

    // I see, .rev() means that the loop goes in reverse
    for idx in (1..4) {
        println!("{idx}");
    }

    let never_value: u32 = loop {
    break 42; // type of never unless it returns something, in this case value is 5
};

println!("{never_value}");
}

// fn while_loop() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }


