// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

// see https://stackoverflow.com/a/67140319/9235421

use crate::mac::my_macro;

fn main() {
    my_macro!();
}

mod mac {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    pub(crate) use my_macro;
}
