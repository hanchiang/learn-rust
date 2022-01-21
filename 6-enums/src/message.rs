// Defining an enum with variants having different amount and types of values
// is similar to defining different structs, except in this case,
// all variants are grouped under the same Message type
// Advantage: Can easily define a function to take a single type that handles all the variants of messages
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("message is {:#?}", self);
    }
}

pub fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}