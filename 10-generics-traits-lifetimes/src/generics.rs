
pub fn main () {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("integer: {:#?}", integer);
    println!("float: {:#?}", float);
    println!("integer.x: {}", integer.x);
}

// 1. Generics in function
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 2. Generics in struct
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

// 3. Generics in enums
// From standard library
enum Option<T> {
    Some(T),
    None
}

// From standard library
enum Result<T, E> {
    OK(T),
    Err(E)
}

// 4. Generics in methods
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Implement methods only on Point<f32>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}