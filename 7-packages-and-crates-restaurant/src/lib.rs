#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod front_of_house;

// by using pub use, external code can now call add_to_waitlist() using host::add_to_waitlist()
pub use crate::front_of_house::hosting;

fn serve_order () {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}



pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

