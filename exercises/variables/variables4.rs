// variables4.rs
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a hint.

#[derive(Debug)]
struct s {
    val: u8,
}

fn main() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
    let st = s { val: 7 };
    println!("struct: {:?}", st);
    println!("struct: {:?}", st);
}
