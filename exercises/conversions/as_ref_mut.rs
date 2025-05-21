// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.

// AI

// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
// Obtain the number of bytes (not characters) in the given argument.
fn byte_counter<T>(arg: T) -> usize
where
    T: AsRef<str>, // This ensures that `arg` can be converted to a reference of type `str`
{
    arg.as_ref().as_bytes().len() // Convert to bytes and count the length
}

// Obtain the number of characters (not bytes) in the given argument.
fn char_counter<T>(arg: T) -> usize
where
    T: AsRef<str>, // This ensures that `arg` can be converted to a reference of type `str`
{
    arg.as_ref().chars().count() // Convert to characters and count them
}

// Squares a number using as_mut().
fn num_sq<T>(arg: &mut T)
where
    T: AsMut<u32>, // This ensures that `arg` can be mutated as a `u32`
{
    // We dereference the mutable reference, get the value, and square it
    let num = arg.as_mut();
    *num *= *num; // Square the value in place
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}

