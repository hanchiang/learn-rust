use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;


pub fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// We want to refer to simulated_expensive_calculation only once in generate_workout,
// but defer the expensive calculation to only where we actually need the result. This is a use case for closures!
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// The Cacher struct has a calculation field of the generic type T.
// The trait bounds on T specify that it’s a closure by using the Fn trait
// The value field is of type Option<u32>. Before we execute the closure,
// value will be None. When code using a Cacher asks for the result of the closure,
// the Cacher will execute the closure at that time and store the result within
// a Some variant in the value field. Then if the code asks for the result of the closure again,
// instead of executing the closure again, the Cacher will return the result held in the Some variant.
struct Cacher<T>
    where T: Fn(u32) -> u32 {
    calculation: T,
    value: HashMap<u32, u32>
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new()
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v1, v1);
        assert_eq!(v2, 2);
    }
}