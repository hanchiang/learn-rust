mod rectangles;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let user = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1
    };

    rectangles::main();
    rectangles::main2();
    rectangles::main3();
}

// ".." is the struct update syntax to copy members of a struct to a new one
// Note that the struct update syntax is like assignment with = because it moves the data
fn build_user_update_syntax(user: User) -> User {
    User {
        email: String::from("some@example.com"),
        ..user
    }
}