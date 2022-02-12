pub fn main() {
    cons_list_mutable();
}

// Use RefCell create a mock Messenger that can mutate &self
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl <'a, T> LimitTracker<'a, T>
where
    T: Messenger
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning! You've used up ove r75% of your quota!");
        }
    }
}

// Combining RefCell with Rc in the using the cons list example
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use std::rc::Rc;
use self::List::{Cons, Nil};
use std::cell::RefCell;

fn cons_list_mutable() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // After we’ve created the lists in a, b, and c, we add 10 to the value in value.
    // We do this by calling borrow_mut on value, which uses the automatic dereferencing feature to dereference
    // the Rc<T> to the inner RefCell<T> value. The borrow_mut method returns a RefMut<T> smart pointer,
    // and we use the dereference operator on it and change the inner value.
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // self needs to be mutably borrowed, but it is defined as immutable in Messenger trait
        // This is a situation in which interior mutability can help.
        // We’ll store the sent_messages within a RefCell<T>, and then the send method will be able to modify sent_messages
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().get(0).unwrap(), "Warning! You've used up ove r75% of your quota!");
    }
}