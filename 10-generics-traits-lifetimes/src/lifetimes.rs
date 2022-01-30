pub fn main() {
    // prevent_dangling_references();
    generic_lifetimes_in_functions();
    generic_lifetimes_in_functions2();
    lifetime_in_struct();

    // references with static lifetime live for the entire duration of the program
    let s: &'static str = "I have a static lifetime.";
}

// Uncomment to get an error: `x` does not live long enough
// This is because the lifetime of x is smaller than r
// fn prevent_dangling_references() {
//     {
//         let r;
//         {
//             let x = 5;
//             r = &x;
//         }
//         println!("r: {}", r);
//     }
// }

fn generic_lifetimes_in_functions() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// pass in references that have different concrete lifetimes
fn generic_lifetimes_in_functions2() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// The return type needs a generic lifetime parameter on it because Rust can’t tell whether
// the reference being returned refers to x or y
// The constraint we want to express in this signature is that the lifetimes of both of the parameters
// and the lifetime of the returned reference are related such that
// the returned reference will be valid as long as both the parameters are
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// There is no need to specify a lifetime on 'y',
// because it does not have any relationship with the lifetime of 'x' or the return value
fn return_first_param<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// lifetime annotations in struct
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn lifetime_in_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}