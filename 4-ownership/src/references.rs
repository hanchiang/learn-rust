pub fn example() {
  let s1 = String::from("hello");

  // `&s1` creates a reference that refers to the value of s1 but does not own it
  // Therefore the value will not be dropped when it goes out of scope
  let len = calculate_length(&s1);

  println!("The length of '{}' is {}.", s1, len);
}

// Function parameter uses the `&` to indicate that s is a reference 
fn calculate_length(s: &String) -> usize {
  s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// Uncomment next 2 functions to produce error
// pub fn modify_reference() {
//   let s = String::from("hello");

//   change(&s);
// }

// fn change(s: &String) {
//   s.push_str(", world!");
// }

pub fn mutable_reference() {
  let mut s = String::from("hello");
  
  println!("Before mutating reference: {}", s);
  change(&mut s);
  println!("After mutating reference: {}", s);
}

fn change(s: &mut String) {
  s.push_str(", world!");
}

// uncomment function to produce error
// pub fn multi_mutable_reference() {
//   let mut s = String::from("hello");

//   let r1 = &mut s;
//   let r2 = &mut s;

//   println!("{}, {}", r1, r2);
// }

// uncomment next 2 functions to produce error
// pub fn dangling_reference() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!
