//Delete input 1 & 2 and have the user only input the nth. The Fibonacci Sequence only starts with..
//0 & 1 (not whichever two numbers you decide to start with).
use std::io;
fn main() {
    println!("Input first number:");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");
    let a: i32 = input1.trim().parse().expect("Input was NOT a number");

    println!("Input second number:");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");
    let b: i32 = input2.trim().parse().expect("Input was NOT a number");

    let mut counter = loop {
        let mut c = a + b;

        let a = b;

        let c = b;
    };
}
//Grok Guide: https://grok.com/c/ee6c9144-bf9c-4888-a771-f610fc956cdf?rid=365371a2-4ac8-4740-a12d-f654ec432515
