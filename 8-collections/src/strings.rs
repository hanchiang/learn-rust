pub fn main() {
    create_string();
    update_string();
    slice_string();
    iterate_string();
}

fn create_string() {
    let s = String::new();

    let data = "initial content";
    let s2 = data.to_string();
    let s3 = "initial content".to_string();
}

fn update_string() {
    let mut s = String::from("foo");
    // double quote for &str
    s.push_str(" bar");
    println!("{}", s);

    let mut s2 = String::from("lo");
    // single quote for char
    s2.push('l');
    println!("{}", s2);

    let s3 = String::from("Hello, ");
    let s4 = String::from("world!");
    // the + operator uses the add method, whose signature look like this
    // fn add(self, s: &str) => String
    // It takes ownership of the calling string, and takes in a &str
    // Rust coerce &s4 into &s4[..] via deref coercion.
    let s5 = s3 + &s4;
    println!("{}", s5);

    let s3 = String::from("hello");
    let s4 = String::from("world");
    let s6 = format!("{}-{}", s3, s4);
    println!("{}", s6);
}

fn slice_string() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s: {}", s);
}

fn iterate_string() {
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}