pub fn main() {
    raw_pointer();
    unsafe_function();
    static_variable();
}

fn raw_pointer() {
    let mut num = 5;

    // We’ve created raw pointers by using as to cast an immutable and a mutable reference into their corresponding raw pointer types.
    // Because we created them directly from references guaranteed to be valid, we know these particular raw pointers are valid,
    // but we can’t make that assumption about just any raw pointer.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // A raw pointer to an arbitrary location in memory. Trying to use arbitrary memory is undefined.
    let address = 0x012345usize;
    let r = address as *const i32;

    // we can create raw pointers in safe code, but we can’t dereference raw pointers and read the data being pointed to
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

fn unsafe_function() {
    unsafe {
        println!("Calling dangerous()");
        dangerous();
    }
}

unsafe fn dangerous() {
    println!("Inside dangerous function");
}

fn extern_function() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// Call C's abs()
extern "C" {
    fn abs(input: i32) -> i32;
}

// Create an interface that allows other languages to call Rust functions
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn static_variable() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
