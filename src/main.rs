use std::io::{self, Read};

fn main() {
    let mut number: i32 = 0;

    println!(
        "<-- FizzBuzz! -->
        Starting from 1, keep counting with the computer until failure.
        If the next number is a multiple of 3, say Fizz instead.
        If the next number is a multiple of 5, say Buzz.
        If your number is a multiple of 3 and 5, say FizzBuzz!
        
        0"
    );

    loop {
        // User Input
        let mut input: String = String::new();

        number += 1;
        let Ok(_) = io::stdin().read_line(&mut input) else {
            return;
        };

        if !(input.trim().to_lowercase() == check_answer(number, "").to_lowercase()) {
            println!("Gotcha! It was {}!", check_answer(number, "!"));
            break;
        };

        number += 1;
        println!("{}", check_answer(number, "!"))
    }

    println!("Press Enter to Exit");
    io::stdin()
        .read_line(&mut String::new())
        .expect("Issue Reading Input but I don\'t care");
}

fn check_answer(num: i32, tack: &str) -> String {
    let mods: (bool, bool) = (num % 3 == 0, num % 5 == 0);
    match mods {
        (false, false) => num.to_string(),
        (true, false) => String::from("Fizz"),
        (false, true) => String::from("Buzz"),
        (true, true) => String::from(format!("FizzBuzz{tack}")),
    }
}
