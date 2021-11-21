fn main() {
    if_else();
    if_let();
    // uncomment next to produce compile error
    // bad_if_let();

    return_value_from_loop();
    while_loop();
    for_loop_range();
    for_loop_iter_collection();
}

fn if_else() {
    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4")
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }
}

fn if_let() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
}

// uncomment function to produce compile error
// fn bad_if_let() {
//     let condition = true;
//     // values in both arms of if condition must return same data type
//     let number = if condition {
//         5
//     } else {
//         "six"
//     };
//     println!("The value of number is: {}", number);
// }

// fn infinite_loop() {
//     loop {
//         println!("again!");
//     }
// }

fn return_value_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        // returns 20 out of the loop
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!");
}

fn for_loop_range() {
    // generates a range from 1 to 3 and reverse it
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}

fn for_loop_iter_collection() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}