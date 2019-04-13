use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // The `Rng` trait defines methods that random number generators implement,
    // and this trait must be in scope via use rand::Rng for us to use those methods
    // The rand::thread_rng function will give us the particular random number generator that we’re going to use:
    // one that is local to the current thread of execution and seeded by the operating system
    // `gen_range` is defined by the `Rng` trait
    // Generates a number in the range [1, 101), default to i32
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // Creates a string by calling the `new` function of the `String` type, and binding it to `guess`
        // Variables are immutable by default
        let mut guess = String::new();

        // The `stdin` function returns an instance of std::io::Stdin, which is the handle to standard input
        // `read_line` reads the standard input and place that into `guess`
        // References, like variables are immutable by default, so need `mut` keyword
        // Handling potential failure with `Result` type, which is an enum
        // It can take `Ok` and `Err` as values.
        // An `Err` value causes the program to crash
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Shadow `guess` variable from above.
        // Shadowing is often used for typecasting operations which allow us to reuse variable name
        // `parse` method parses a string into a number. Because this method can parse a variety of number types, 
        // we need to tell Rust the exact number type we want by using `u32`
        // This means Rust will infer that `secret_number` should be a `u32` as well
        // Use `match` expression to handle invalid inputs instead of crashing with `expect` if `Err` is returned
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // `Ordering` is another enum, with values `Less`, `Greater`, `Equal`
        // Use `match` expression to determine actions based on the variant of `Ordering` returned by `cmp`
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}