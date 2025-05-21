// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

// AI

macro_rules! my_macro {
    ($msg:expr) => {
        println!("Check out my macro! Message: {}", $msg);
    };
}

fn main() {
    my_macro!("Hello, Rust!");
}
