// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

// AI
mod macros {
    // Define the macro inside the macros module
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    // Make sure to export the macro so it's accessible outside this module
    pub(crate) use my_macro;
}

fn main() {
    // Use the macro that has been brought into scope
    use macros::my_macro;
    my_macro!(); // This works now
}

