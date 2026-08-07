//Handle user input error (Have the program ask for another nth if the user inputs an invalid
//character (better than the program crashing after an invalid input))
use std::io;
fn main() {
    println!(
        "Input an nth (e.g. '3' (the third number in order of the Fibonacci Sequence will be 1)))"
    );
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    let input: i32 = input.trim().parse().expect("Error: Please type a number");

    let mut counter = 0;

    loop {
        counter += 1;

        println!("{counter}");

        if counter == 50 {
            break;
        }
    }
}
//Grok Guide: https://grok.com/c/ee6c9144-bf9c-4888-a771-f610fc956cdf?rid=365371a2-4ac8-4740-a12d-f654ec432515
