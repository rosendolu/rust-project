// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    call_me(u32::MAX);
}

fn call_me(num: u32) {
    for i in num - 1..=num {
        println!("Ring! Call number {:?}", i.wrapping_add(1));
    }
}
