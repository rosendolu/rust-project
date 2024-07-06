// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
    ($val:expr , $val2:expr) => {
        println!("Look at this other macro: {} {}", $val,$val2);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
    my_macro!(7777, 888);
}
