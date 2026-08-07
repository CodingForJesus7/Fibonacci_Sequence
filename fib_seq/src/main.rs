//Handle user input error (Have the program ask for another nth if the user inputs an invalid
//character (better than the program crashing after an invalid input))

//Have program go back to starter prompt after each outputted nth
use std::io;
fn main() {
    loop {
        println!(
            "Input an nth (e.g. '3' (the third number in order of the Fibonacci Sequence will be 1))"
        );
        println!("Press q to quit program");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let input = input.trim();
        if { input } == "q" {
            break;
        }

        let input: i32 = match input.parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please type a number (or 'q' to quit)");
                continue;
            }
        };

        let mut counter = 2;

        let mut a = 0;

        let mut b = 1;

        //I gave c the value of 0 as a placeholder
        let mut c = 0;

        loop {
            if { input } == 1 || { input } == 2 {
                if { input } == 1 {
                    println!("0 is the {input} nth");

                    break;
                }
                if { input } == 2 {
                    println!("1 is the {input} nth");

                    break;
                }
            } else {
                b = a + b;
                a = b - a;

                counter += 1;

                if counter == { input } {
                    println!("{b} is the {input} nth");

                    break;
                }
            }
        }
    }
}
//Grok Guide: https://grok.com/c/ee6c9144-bf9c-4888-a771-f610fc956cdf?rid=365371a2-4ac8-4740-a12d-f654ec432515
