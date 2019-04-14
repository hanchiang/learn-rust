fn main() {
    value_moved();

    value_copied();
    value_cloned();

    /**
     * Function ownership
     */
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);
    // s is no longer valid here...

    // uncomment next line to produce error
    // println!("s: {}", s);

    let x = 5;  // x comes into scope
    makes_copy(x);  // x move into `makes_copy`, but it is still valid afterwards because i32 is Copy
    println!("x is: {}", x);

    /**
     * Return values and scopes
     */
    let s1 = gives_ownership(); // moves return value to s1
    println!("s1: {}", s1);

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into `takes_and_gives_back`, whose return value is moved into s3
    println!("s3: {}", s3);

    // uncomment next line to produce error
    // println!("s2: {}", s2);
}

// uncomment function to produce error
fn value_moved() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s2);
    
    // Results in error because value of s1 is moved to s2, and s1 is invalid
    // because the pointer, not the data, is copied during assignment for heap-allocated data
    // uncomment next line to produce error
    // println!("{}, world!", s1);
}

// This function compiles because primitive data values (integer, floating point, boolean, character, array, tuple)
// are stored on the stack, and therefore copied during assignment
fn value_copied() {
    let a = 3;
    let b = a;
    println!("a: {}, b: {}", a, b);
}

// Function compiles because heap data of s1 is deep copied to s2
// cloning heap data may be an expensive operation, depending on the size of the data copied
fn value_cloned() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and `drop` is called

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the caller function
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string //a_string is returned and moves out to the caller function
}