
pub fn main() {
    basic();
}

// We define the variable b to have the value of a Box that points to the value 5,
// which is allocated on the heap. This program will print b = 5; in this case,
// we can access the data in the box similar to how we would if this data were on the stack.
// Just like any owned value, when a box goes out of scope,
// as b does at the end of main, it will be deallocated.
// The deallocation happens for the box (stored on the stack) and the data it points to (stored on the heap).
fn basic() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// Let’s look at a case where boxes allow us to define types that we wouldn’t be allowed to if we didn’t have boxes.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use self::List::{Cons, Nil};

fn cons() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
