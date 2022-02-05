mod closure;
mod iterator;

use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;


fn main() {
    closure::main();
    iterator::main();
}
