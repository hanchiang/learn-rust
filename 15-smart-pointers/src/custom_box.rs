use std::ops::Deref;

pub fn main() {
    deref();
    my_drop();
}

// A tuple struct with 1 element
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn deref() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // internally, *y is converted to *(y.deref())
    assert_eq!(5, *y);


    let m = MyBox::new(String::from("Rust"));
    // Because we implemented the Deref trait on MyBox<T>, Rust can turn &MyBox<String> into &String by calling deref.
    // The standard library provides an implementation of Deref on String that returns a string slice.
    // Rust calls deref again to turn the &String into &str, which matches the hello function’s definition.
    // If Rust didn’t implement deref coercion, we would have to write this:
    // hello(&(*m)[..]);
    // (*m) dereferences MyBox<String> into a String. & and [..] take a string slice of the String
    // that is equal to the whole string to match the signature of hello().
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}



struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn my_drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff")
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff")
    };
    println!("CustomSmartPointers created.");
    drop(c);
    println!("c dropped before the end of function.");
}



