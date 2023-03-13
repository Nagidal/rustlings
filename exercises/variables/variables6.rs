// variables6.rs
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a hint.

const NUMBER: usize = 3;
static mut x: usize = 6;
fn main() {
    println!("Number {}", NUMBER);
    unsafe {
        x = 8;
    };
}
