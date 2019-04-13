fn main() {
    println!("Hello, world!");
    my_func(2);
    another_func(3, 5);

    let x = plus_one(3);
    println!("after plus_one: {}", x);

    // uncomment next line to produce compile error
    // let y = bad_plus_one(2);

}

fn my_func(x: i32) {
    println!("my_func received i32 integer: {}", x)
}

fn another_func(x: i32, y: u32) {
    println!("another_func received i32 and u32 integer: {}, {}", x, y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn bad_plus_one(x: i32) -> i32 {
    x + 1;
}