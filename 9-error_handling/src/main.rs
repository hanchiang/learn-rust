mod unrecoverable_error;
mod recoverable_error;

fn main() {
    // unrecoverable_error::crash();
    // unrecoverable_error::panic_backtrace_from_library();
    recoverable_error::open_file();
    recoverable_error::open_file_better();
}

