use std::io;
fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = input.trim().parse().expect("Input was NOT a number");

    let sel = [0, 1, 1, 2, 3, 5, 8, 13];
    //"sel" is short for selection
}
