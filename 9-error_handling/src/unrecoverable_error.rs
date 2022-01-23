pub fn crash() {
    panic!("crash and burn");
}

pub fn panic_backtrace_from_library() {
    let v = vec![1, 2, 3];

    v[99];
}