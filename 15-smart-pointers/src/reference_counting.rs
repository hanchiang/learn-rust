enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;
use self::List::{Cons, Nil};

pub fn main() {
    basic();
    count();
}

fn basic() {
    // 3 references to the list: 5 -> 10 > Nil
    // The implementation of Rc::clone doesn’t make a deep copy of all the data like most types’ implementations of clone do.
    // The call to Rc::clone only increments the reference count, which doesn’t take much time.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

fn count() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}